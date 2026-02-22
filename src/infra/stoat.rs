use crate::config::AppConfig;
use crate::domain::{models::*, ports::*};
use ::anyhow::{Context, Result};
use ::futures_util::{SinkExt, StreamExt};
use ::reqwest::Client;
use ::serde_json::{json, Value};
use ::std::{collections::HashMap, sync::Arc};
use ::tokio::sync::{mpsc, RwLock};

pub struct Stoat {
    http: Client,
    token: ::std::string::String,
    log: ::std::string::String,
    props: Arc<RwLock<HashMap<::std::string::String, ::std::string::String>>>,
}

impl Stoat {
    pub async fn new(token: &str, log: &str, staff: &str, tx: mpsc::Sender<Event>, config: Arc<AppConfig>) -> Result<Self> {
        let http = Client::builder().user_agent("Urchin").danger_accept_invalid_certs(true).build()?;
        let props = Arc::new(RwLock::new(HashMap::new()));
        
        let (tk, ht, st, pr, txc, cfg) = (token.to_string(), http.clone(), staff.to_string(), props.clone(), tx, config);
        ::tokio::spawn(async move {
            loop {
                if let Err(e) = Self::listen(&ht, &tk, &st, &txc, &pr, &cfg).await { ::tracing::error!("Stoat WS: {}", e); }
                ::tokio::time::sleep(::std::time::Duration::from_secs(5)).await;
            }
        });

        Ok(Self { http, token: token.into(), log: log.into(), props })
    }

    async fn listen(http: &Client, tk: &str, staff: &str, tx: &mpsc::Sender<Event>, props: &Arc<RwLock<HashMap<::std::string::String, ::std::string::String>>>, cfg: &Arc<AppConfig>) -> Result<()> {
        let tls = ::native_tls::TlsConnector::builder().danger_accept_invalid_certs(true).build()?;
        let (ws, _) = ::tokio_tungstenite::connect_async_tls_with_config("wss://stoat.chat/events", None, false, Some(::tokio_tungstenite::Connector::NativeTls(tls))).await?;
        let (mut w, mut r) = ws.split();

        w.send(::tokio_tungstenite::tungstenite::Message::Text(json!({"type": "Authenticate", "token": tk}).to_string())).await?;
        let mut hb = ::tokio::time::interval(::std::time::Duration::from_secs(20));

        loop {
            ::tokio::select! {
                _ = hb.tick() => { w.send(::tokio_tungstenite::tungstenite::Message::Text(json!({"type": "Ping", "data": 0}).to_string())).await?; }
                msg = r.next() => {
                    let pl: Value = ::serde_json::from_str(&msg.context("WS End")??.into_text()?)?;
                    match pl["type"].as_str() {
                        Some("Authenticated") => { w.send(::tokio_tungstenite::tungstenite::Message::Text(json!({"type": "UpdateUser", "data": {"status": {"presence": "Online"}}}).to_string())).await?; }
                        Some("MessageReact") => { Self::on_react(&pl, http, tk, staff, tx, props).await?; }
                        Some("Message") => { Self::on_msg(&pl, &cfg.command_prefix, tx).await?; }
                        _ => {}
                    }
                }
            }
        }
    }

    async fn on_msg(pl: &Value, prefix: &str, tx: &mpsc::Sender<Event>) -> Result<()> {
        let content = pl["content"].as_str().unwrap_or("");
        let parts: ::std::vec::Vec<&str> = content.split_whitespace().collect();
        if parts.len() >= 3 && parts[0].starts_with(prefix) {
            tx.send(Event::Propose {
                action: parts[0][prefix.len()..].to_string(),
                target: parts[1].to_string(),
                reason: parts[2..].join(" "),
                author: pl["author"].as_str().unwrap_or("?").into(),
                origin: Platform::Stoat,
                channel: pl["channel"].as_str().unwrap_or("?").into()
            }).await?;
        }
        Ok(())
    }

    async fn on_react(pl: &Value, http: &Client, tk: &str, staff: &str, tx: &mpsc::Sender<Event>, props: &Arc<RwLock<HashMap<::std::string::String, ::std::string::String>>>) -> Result<()> {
        let is_ok = pl["emoji_id"].as_str().is_some_and(|e| e.contains('✅'));
        let is_no = pl["emoji_id"].as_str().is_some_and(|e| e.contains('❌'));
        if !is_ok && !is_no { return Ok(()); }

        let uid = pl["user_id"].as_str().context("Missing user_id")?;
        let mid = pl["id"].as_str().context("Missing id")?;
        let cid = pl["channel_id"].as_str().context("Missing channel_id")?;
        
        let chan: Value = http.get(format!("https://stoat.chat/api/channels/{}", cid)).header("x-bot-token", tk).send().await?.json().await?;
        let sid = chan["server"].as_str().context("No server")?;
        let mem: Value = http.get(format!("https://stoat.chat/api/servers/{}/members/{}", sid, uid)).header("x-bot-token", tk).send().await?.json().await?;
        
        if mem["roles"].as_array().is_some_and(|r| r.iter().any(|v| v.as_str() == Some(staff))) {
            if let Some(target) = props.read().await.get(mid) {
                if is_ok { tx.send(Event::Approve { target: target.clone(), approver: uid.into() }).await?; }
                if is_no { tx.send(Event::Cancel { target: target.clone(), author: uid.into() }).await?; }
            }
        }
        Ok(())
    }
}

#[::async_trait::async_trait]
impl Driver for Stoat {
    async fn notify(&self, p: &Proposal) -> Result<()> {
        let ch = if p.origin == Platform::Stoat { &p.channel } else { &self.log };
        let msg = format!("**TPI {}**\nTarget: {}\nReq: {}\nReason: {}\n_React ✅ to approve, ❌ to cancel_", p.action, p.target.stoat.as_deref().unwrap_or(&p.target.raw), p.author, p.reason);
        let res: Value = self.http.post(format!("https://stoat.chat/api/channels/{}/messages", ch)).header("x-bot-token", &self.token).json(&json!({"content": msg})).send().await?.json().await?;
        
        if let Some(id) = res["_id"].as_str() {
            let _ = self.http.put(format!("https://stoat.chat/api/channels/{}/messages/{}/reactions/✅", ch, id)).header("x-bot-token", &self.token).send().await;
            let _ = self.http.put(format!("https://stoat.chat/api/channels/{}/messages/{}/reactions/❌", ch, id)).header("x-bot-token", &self.token).send().await;
            self.props.write().await.insert(id.into(), p.target.raw.clone());
        }
        Ok(())
    }

    async fn execute(&self, p: &Proposal, app: &str) -> Result<()> {
        let ch = if p.origin == Platform::Stoat { &p.channel } else { &self.log };
        self.http.post(format!("https://stoat.chat/api/channels/{}/messages", ch)).header("x-bot-token", &self.token).json(&json!({"content": format!("✅ Executed {} on {} (App: {})", p.action, p.target.raw, app)})).send().await?;
        Ok(())
    }

    async fn discard(&self, p: &Proposal, reason: &str) -> Result<()> {
        let ch = if p.origin == Platform::Stoat { &p.channel } else { &self.log };
        self.http.post(format!("https://stoat.chat/api/channels/{}/messages", ch)).header("x-bot-token", &self.token).json(&json!({"content": format!("🚫 {} discarded: {}", p.action, reason)})).send().await?;
        Ok(())
    }
}
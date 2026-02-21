use ::std::collections::HashMap;
use ::std::sync::Arc;
use ::anyhow::{Context, Result};
use ::async_trait::async_trait;
use ::tokio::sync::{mpsc, RwLock};
use ::tracing::{error, info};

use ::reqwest::Client as HttpClient;
use ::serde_json::{json, Value};
use ::tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use ::futures_util::{SinkExt, StreamExt};

use crate::domain::ports::{BanProposal, PlatformNotifier, UrchinEvent, Platform};

pub struct StoatAdapter {
    http: HttpClient,
    token: ::std::string::String,
    log_channel_id: ::std::string::String,
    active_proposals: Arc<RwLock<HashMap<::std::string::String, ::std::string::String>>>,
}

unsafe impl Send for StoatAdapter {}
unsafe impl Sync for StoatAdapter {}

impl StoatAdapter {
    pub async fn new(
        token: &str,
        log_channel_id: ::std::string::String,
        staff_role_id: ::std::string::String,
        tx: mpsc::Sender<UrchinEvent>,
    ) -> Result<Self> {
        let http = HttpClient::builder().build().context("Failed to build reqwest client")?;
        let active_proposals = Arc::new(RwLock::new(HashMap::<::std::string::String, ::std::string::String>::new()));
        
        let token_clone = token.to_string();
        let proposals_clone = Arc::clone(&active_proposals);
        let http_clone = http.clone();
        
        ::tokio::spawn(async move {
            loop {
                let ws_url = "wss://ws.revolt.chat?version=1&format=json";
                
                match connect_async(ws_url).await {
                    Ok((ws_stream, _)) => {
                        info!("Connected to Stoat WebSocket Gateway");
                        let (mut write, mut read) = ws_stream.split();
                        
                        let auth_payload = json!({
                            "type": "Authenticate",
                            "token": token_clone
                        }).to_string();

                        if let Err(e) = write.send(Message::Text(auth_payload)).await {
                            error!("Failed to authenticate on Stoat WS: {}", e);
                            ::tokio::time::sleep(::std::time::Duration::from_secs(5)).await;
                            continue;
                        }

                        let mut ping_interval = ::tokio::time::interval(::std::time::Duration::from_secs(15));
                        
                        loop {
                            ::tokio::select! {
                                _ = ping_interval.tick() => {
                                    let ping = json!({"type": "Ping", "data": 0}).to_string();
                                    if write.send(Message::Text(ping)).await.is_err() {
                                        break;
                                    }
                                }
                                msg_opt = read.next() => {
                                    match msg_opt {
                                        Some(Ok(Message::Text(text))) => {
                                            if let Ok(parsed) = ::serde_json::from_str::<Value>(&text) {
                                                if parsed["type"] == "MessageReact" {
                                                    let emoji = parsed["emoji_id"].as_str().unwrap_or("");
                                                    let user_id = parsed["user_id"].as_str().unwrap_or("");
                                                    let message_id = parsed["id"].as_str().unwrap_or("");
                                                    let channel_id = parsed["channel_id"].as_str().unwrap_or("");
                                                    
                                                    if emoji.contains("✅") {
                                                        if let Ok(ch_res) = http_clone.get(format!("https://api.revolt.chat/channels/{}", channel_id))
                                                            .header("x-bot-token", &token_clone).send().await {
                                                            
                                                            if let Ok(ch_data) = ch_res.json::<Value>().await {
                                                                if let Some(server_id) = ch_data["server"].as_str() {
                                                                    if let Ok(m_res) = http_clone.get(format!("https://api.revolt.chat/servers/{}/members/{}", server_id, user_id))
                                                                        .header("x-bot-token", &token_clone).send().await {
                                                                        
                                                                        if let Ok(member) = m_res.json::<Value>().await {
                                                                            let roles = member["roles"].as_array();
                                                                            let has_role = roles.is_some_and(|r| r.iter().any(|v| v.as_str() == Some(&staff_role_id)));
                                                                            
                                                                            if !has_role {
                                                                                info!(user_id = %user_id, "Unauthorized approval attempt rejected.");
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }

                                                        let map = proposals_clone.read().await;
                                                        if let Some(target) = map.get(message_id) {
                                                            let _ = tx.send(UrchinEvent::ConfirmAction {
                                                                target: target.clone(),
                                                                approver: user_id.to_string(),
                                                            }).await;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(Ok(_)) => {}, 
                                        Some(Err(e)) => {
                                            error!("Stoat WS stream error: {}", e);
                                            break;
                                        }
                                        None => {
                                            info!("Stoat WS stream closed by gateway");
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => error!("Failed to establish Stoat WebSocket connection: {}", e),
                }

                error!("Stoat connection lost. Reconnecting in 5 seconds...");
                ::tokio::time::sleep(::std::time::Duration::from_secs(5)).await;
            }
        });

        Ok(Self {
            http,
            token: token.to_string(),
            log_channel_id,
            active_proposals,
        })
    }
}

#[async_trait]
impl PlatformNotifier for StoatAdapter {
    async fn notify_proposal(&self, proposal: &BanProposal) -> Result<()> {
        let channel = if proposal.origin_platform == Platform::Stoat {
            &proposal.origin_channel_id
        } else {
            &self.log_channel_id
        };

        let payload = format!(
            "**TPI Action Required:**\nTarget: {}\nReason: {}\nOrigin: {:?}\n\n_React with ✅ to approve._", 
            proposal.target_id, 
            proposal.reason,
            proposal.origin_platform
        );
        
        let res = self.http.post(format!("https://api.revolt.chat/channels/{}/messages", channel))
            .header("x-bot-token", &self.token)
            .json(&json!({"content": payload}))
            .send()
            .await?;

        if let Ok(body) = res.json::<Value>().await {
            if let Some(msg_id) = body["_id"].as_str() {
                let _ = self.http.put(format!("https://api.revolt.chat/channels/{}/messages/{}/reactions/✅", channel, msg_id))
                    .header("x-bot-token", &self.token)
                    .send()
                    .await;

                self.active_proposals.write().await.insert(msg_id.to_string(), proposal.target_id.clone());
            }
        }
        
        Ok(())
    }

    async fn execute_action(&self, proposal: &BanProposal, approver: &str) -> Result<()> {
        let channel = if proposal.origin_platform == Platform::Stoat {
            &proposal.origin_channel_id
        } else {
            &self.log_channel_id
        };

        let payload = format!(
            "**Action Executed:**\nTarget: {} handled by {} (Approved by {})", 
            proposal.target_id, 
            proposal.requester_id, 
            approver
        );
        
        let _ = self.http.post(format!("https://api.revolt.chat/channels/{}/messages", channel))
            .header("x-bot-token", &self.token)
            .json(&json!({"content": payload}))
            .send()
            .await;
            
        Ok(())
    }
}
use super::{models::*, ports::*};
use crate::config::AppConfig;
use ::std::sync::Arc;
use ::std::time::{SystemTime, UNIX_EPOCH};

pub struct Core {
    store: Arc<dyn StateStore>,
    drivers: ::std::vec::Vec<Arc<dyn Driver>>,
    config: Arc<AppConfig>,
}

impl Core {
    pub fn new(store: Arc<dyn StateStore>, drivers: ::std::vec::Vec<Arc<dyn Driver>>, config: Arc<AppConfig>) -> Self {
        Self { store, drivers, config }
    }

    pub async fn run(&self, event: Event) -> ::anyhow::Result<()> {
        match event {
            Event::Propose { action, target, author, origin, channel, reason } => {
                let clean_target = target.replace("<@", "").replace(">", "").replace("!", "");
                let is_discord = clean_target.chars().all(|c| c.is_ascii_digit()) && clean_target.len() >= 17;
                let is_stoat = clean_target.len() == 26 && clean_target.chars().all(|c| c.is_ascii_alphanumeric());

                let id = Identity {
                    raw: clean_target.clone(),
                    discord: if is_discord || origin == Platform::Discord { Some(clean_target.clone()) } else { None },
                    stoat: if is_stoat || origin == Platform::Stoat { Some(clean_target.clone()) } else { None },
                };
                
                let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                let p = Proposal { target: id, action, reason, author, origin, channel, approvers: ::std::vec::Vec::new(), timestamp: ts };
                
                self.store.save(p.clone()).await?;
                for d in &self.drivers {
                    if let Err(e) = d.notify(&p).await { ::tracing::error!("Driver Notify Error: {}", e); }
                }
            }
            Event::Approve { target, approver } => {
                if let Some(mut p) = self.store.get(&target).await? {
                    if p.author == approver { ::anyhow::bail!("Self-approval rejected."); }
                    if !p.approvers.contains(&approver) { p.approvers.push(approver.clone()); }

                    if p.approvers.len() >= self.config.required_approvals {
                        for d in &self.drivers {
                            if let Err(e) = d.execute(&p, &approver).await { ::tracing::error!("Driver Execute Error: {}", e); }
                        }
                        self.store.remove(&target).await?;
                    } else {
                        self.store.save(p).await?;
                    }
                }
            }
            Event::Cancel { target, author } => {
                if let Some(p) = self.store.get(&target).await? {
                    for d in &self.drivers {
                        if let Err(e) = d.discard(&p, &format!("Cancelled by {}", author)).await {
                            ::tracing::error!("Driver Discard Error: {}", e);
                        }
                    }
                    self.store.remove(&target).await?;
                }
            }
            Event::Sweep => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                for p in self.store.list().await? {
                    if now > p.timestamp + self.config.expiry_seconds {
                        for d in &self.drivers {
                            let _ = d.discard(&p, "Expired").await;
                        }
                        self.store.remove(&p.target.raw).await?;
                    }
                }
            }
        }
        Ok(())
    }
}
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
                let id = Identity {
                    raw: target.clone(),
                    discord: if origin == Platform::Discord { Some(target.clone()) } else { None },
                    stoat: if origin == Platform::Stoat { Some(target.clone()) } else { None },
                };
                let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                let p = Proposal { target: id, action, reason, author, origin, channel, approvers: ::std::vec::Vec::new(), timestamp: ts };
                
                self.store.save(p.clone()).await?;
                for d in &self.drivers { let _ = d.notify(&p).await; }
            }
            Event::Approve { target, approver } => {
                if let Some(mut p) = self.store.get(&target).await? {
                    if p.author == approver { ::anyhow::bail!("Self-approval rejected."); }
                    if !p.approvers.contains(&approver) { p.approvers.push(approver.clone()); }

                    if p.approvers.len() >= self.config.required_approvals {
                        for d in &self.drivers { let _ = d.execute(&p, &approver).await; }
                        self.store.remove(&target).await?;
                    } else {
                        self.store.save(p).await?;
                    }
                }
            }
            Event::Cancel { target, author } => {
                if let Some(p) = self.store.get(&target).await? {
                    if p.author != author { ::anyhow::bail!("Only proposer can cancel."); }
                    for d in &self.drivers { let _ = d.discard(&p, "Cancelled by author").await; }
                    self.store.remove(&target).await?;
                }
            }
            Event::Sweep => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                for p in self.store.list().await? {
                    if now > p.timestamp + self.config.expiry_seconds {
                        for d in &self.drivers { let _ = d.discard(&p, "Expired").await; }
                        self.store.remove(&p.target.raw).await?;
                    }
                }
            }
        }
        Ok(())
    }
}
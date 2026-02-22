use super::{models::*, ports::*};
use std::sync::Arc;

pub struct Core {
    store: Arc<dyn StateStore>,
    drivers: Vec<Arc<dyn Driver>>,
}

impl Core {
    pub fn new(store: Arc<dyn StateStore>, drivers: Vec<Arc<dyn Driver>>) -> Self {
        Self { store, drivers }
    }

    pub async fn run(&self, event: Event) -> anyhow::Result<()> {
        match event {
            Event::Propose { action, target, author, origin, channel, reason } => {
                let id = Identity {
                    raw: target.clone(),
                    discord: if origin == Platform::Discord { Some(target.clone()) } else { None },
                    stoat: if origin == Platform::Stoat { Some(target.clone()) } else { None },
                };
                let p = Proposal { target: id, action, reason, author, origin, channel };
                
                self.store.save(p.clone()).await?;
                for d in &self.drivers { let _ = d.notify(&p).await; }
            }
            Event::Approve { target, approver } => {
                if let Some(p) = self.store.get(&target).await? {
                    if p.author == approver { anyhow::bail!("Self-approval rejected."); }
                    for d in &self.drivers { let _ = d.execute(&p, &approver).await; }
                    self.store.remove(&target).await?;
                }
            }
        }
        Ok(())
    }
}
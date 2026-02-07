use async_trait::async_trait;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BanProposal {
    pub target_id: String,
    pub reason: String,
    pub requester_id: String,
}

#[async_trait]
pub trait BanRepository: Send + Sync {
    async fn save_proposal(&self, proposal: BanProposal) -> Result<()>;
    async fn get_proposal(&self, target_id: &str) -> Result<Option<BanProposal>>;
}

#[async_trait]
pub trait PlatformNotifier: Send + Sync {
    async fn broadcast_alert(&self, message: &str) -> Result<()>;
}
use async_trait::async_trait;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct BanProposal {
    pub target_id: String,
    pub reason: String,
    pub requester_id: String,
    pub channel_id: String,
    pub kind: String,
}

#[derive(Debug)]
pub enum UrchinEvent {
    RequestAction { kind: String, target: String, requester: String, channel_id: String, reason: String },
    ConfirmAction { target: String, approver: String },
}

#[async_trait]
pub trait BanRepository: Send + Sync {
    async fn save_proposal(&self, proposal: BanProposal) -> Result<()>;
    async fn get_proposal(&self, target_id: &str) -> Result<Option<BanProposal>>;
    async fn delete_proposal(&self, target_id: &str) -> Result<()>;
}

#[async_trait]
pub trait PlatformNotifier: Send + Sync {
    async fn notify_proposal(&self, proposal: &BanProposal) -> Result<()>;
    async fn execute_action(&self, proposal: &BanProposal, approver: &str) -> Result<()>;
}
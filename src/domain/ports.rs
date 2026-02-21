use ::async_trait::async_trait;
use ::anyhow::Result;
use ::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Discord,
    Stoat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanProposal {
    pub target_id: ::std::string::String,
    pub reason: ::std::string::String,
    pub requester_id: ::std::string::String,
    pub origin_platform: Platform,
    pub origin_channel_id: ::std::string::String,
    pub kind: ::std::string::String,
}

#[derive(Debug)]
pub enum UrchinEvent {
    RequestAction {
        kind: ::std::string::String,
        target: ::std::string::String,
        requester: ::std::string::String,
        origin_platform: Platform,
        origin_channel_id: ::std::string::String,
        reason: ::std::string::String,
    },
    ConfirmAction {
        target: ::std::string::String,
        approver: ::std::string::String,
    },
}

#[async_trait]
pub trait BanRepository: Send + Sync {
    async fn save_proposal(&self, proposal: BanProposal) -> Result<()>;
    async fn get_proposal(&self, target_id: &str) -> Result<::std::option::Option<BanProposal>>;
    async fn delete_proposal(&self, target_id: &str) -> Result<()>;
}

#[async_trait]
pub trait PlatformNotifier: Send + Sync {
    async fn notify_proposal(&self, proposal: &BanProposal) -> Result<()>;
    async fn execute_action(&self, proposal: &BanProposal, approver: &str) -> Result<()>;
}
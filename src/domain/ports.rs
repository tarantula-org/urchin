use super::models::Proposal;
use ::anyhow::Result;

#[::async_trait::async_trait]
pub trait StateStore: Send + Sync {
    async fn save(&self, p: Proposal) -> Result<()>;
    async fn get(&self, target: &str) -> Result<::std::option::Option<Proposal>>;
    async fn remove(&self, target: &str) -> Result<()>;
    async fn list(&self) -> Result<::std::vec::Vec<Proposal>>;
}

#[::async_trait::async_trait]
pub trait Driver: Send + Sync {
    async fn notify(&self, p: &Proposal) -> Result<()>;
    async fn execute(&self, p: &Proposal, approver: &str) -> Result<()>;
    async fn discard(&self, p: &Proposal, reason: &str) -> Result<()>;
}
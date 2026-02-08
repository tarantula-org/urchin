use async_trait::async_trait;
use teloxide::prelude::*;
use crate::domain::ports::{PlatformNotifier, BanProposal};

pub struct TelegramAdapter {
    _bot: Bot,
}

impl TelegramAdapter {
    pub async fn new(token: &str) -> anyhow::Result<Self> {
        let bot = Bot::new(token); 
        Ok(Self { _bot: bot })
    }
}

#[async_trait]
impl PlatformNotifier for TelegramAdapter {
    async fn notify_proposal(&self, _proposal: &BanProposal) -> anyhow::Result<()> {
        Ok(())
    }

    async fn execute_action(&self, _proposal: &BanProposal, _approver: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
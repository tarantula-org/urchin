use async_trait::async_trait;
use teloxide::prelude::*;
use crate::domain::ports::PlatformNotifier;

pub struct TelegramAdapter {
    _bot: Bot,
}

impl TelegramAdapter {
    pub async fn new(_token: &str) -> anyhow::Result<Self> {
        let bot = Bot::from_env(); 
        Ok(Self { _bot: bot })
    }
}

#[async_trait]
impl PlatformNotifier for TelegramAdapter {
    async fn broadcast_alert(&self, message: &str) -> anyhow::Result<()> {
        println!("[Telegram] {}", message);
        Ok(())
    }
}
use async_trait::async_trait;
use serenity::all::{GatewayIntents, Client};
use crate::domain::ports::PlatformNotifier;

pub struct DiscordAdapter {
    _client: Client,
}

impl DiscordAdapter {
    pub async fn new(token: &str) -> anyhow::Result<Self> {
        let intents = GatewayIntents::GUILD_MESSAGES;
        let client = Client::builder(token, intents).await?;
        Ok(Self { _client: client })
    }
}

#[async_trait]
impl PlatformNotifier for DiscordAdapter {
    async fn broadcast_alert(&self, message: &str) -> anyhow::Result<()> {
        println!("[Discord] {}", message);
        Ok(())
    }
}
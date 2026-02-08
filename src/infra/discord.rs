use async_trait::async_trait;
use serenity::all::{GatewayIntents, Client, Http, ChannelId};
use std::sync::Arc;
use crate::domain::ports::PlatformNotifier;

pub struct DiscordAdapter {
    http: Arc<Http>,   // For sending messages (Stateless)
    _client: Client,   // For receiving events (Stateful Gateway)
    channel_id: u64,   // Where to send alerts
}

impl DiscordAdapter {
    pub async fn new(token: &str, target_channel_id: u64) -> anyhow::Result<Self> {
        // 1. Setup Client (Listener)
        let intents = GatewayIntents::GUILD_MESSAGES;
        let client = Client::builder(token, intents).await?;

        let http = Arc::new(Http::new(token));

        Ok(Self { 
            http, 
            _client: client,
            channel_id: target_channel_id 
        })
    }
}

#[async_trait]
impl PlatformNotifier for DiscordAdapter {
    async fn broadcast_alert(&self, message: &str) -> anyhow::Result<()> {
        let channel = ChannelId::new(self.channel_id);
        
        // This actually sends the network request to Discord
        channel.say(&self.http, message).await?;
        
        println!("[Discord] Sent alert to channel {}", self.channel_id);
        Ok(())
    }
}
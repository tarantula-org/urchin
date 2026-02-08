mod domain;
mod infra;

use std::sync::Arc;
use std::env;
use anyhow::Result;
use dotenvy::dotenv; // Load .env

use infra::discord::DiscordAdapter;
use infra::telegram::TelegramAdapter;
use infra::persistence::SledRepository;
use domain::services::ConsensusService;
use domain::ports::PlatformNotifier;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load Secrets
    dotenv().ok(); // Ignore error if .env is missing (e.g. in prod)
    
    let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");
    let discord_channel_id = env::var("DISCORD_CHANNEL_ID")
        .expect("DISCORD_CHANNEL_ID must be set")
        .parse::<u64>()
        .expect("Invalid Channel ID");

    // 2. Adapters
    // Pass the channel ID to the adapter
    let discord = DiscordAdapter::new(&discord_token, discord_channel_id).await?;
    
    // For Telegram, we keep the stub for now (or add a dummy token)
    let telegram = TelegramAdapter::new("dummy_token").await?;
    let db = SledRepository::new("./urchin_db")?;

    // 3. Wiring
    let notifiers: Vec<Arc<dyn PlatformNotifier>> = vec![
        Arc::new(discord),
        Arc::new(telegram),
    ];

    let engine = ConsensusService::new(Arc::new(db), notifiers);

    println!("Urchin Nucleus Active.");

    // 4. Live Test
    println!("--- Sending Test Alert to Discord ---");
    
    // This will now cause your Discord Bot to actually post in the channel!
    engine.request_ban("User_Test_123", "Admin_Acrilic").await?;

    Ok(())
}
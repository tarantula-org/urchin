mod domain;
mod infra;

use std::sync::Arc;
use std::env;
use anyhow::Result;
use dotenvy::dotenv;
use tokio::sync::mpsc;

use infra::discord::DiscordAdapter;
use infra::telegram::TelegramAdapter;
use infra::persistence::SledRepository;
use domain::services::ConsensusService;
use domain::ports::{PlatformNotifier, UrchinEvent};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    let (tx, mut rx) = mpsc::channel::<UrchinEvent>(100);

    let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN missing");
    let discord_guild_id = env::var("DISCORD_GUILD_ID")
        .expect("DISCORD_GUILD_ID missing")
        .parse::<u64>()?;
    let discord_staff_role = env::var("DISCORD_STAFF_ROLE_ID")
        .expect("DISCORD_STAFF_ROLE_ID missing")
        .parse::<u64>()?;
    
    let discord = DiscordAdapter::new(&discord_token, discord_guild_id, discord_staff_role, tx).await?;
    let telegram = TelegramAdapter::new("dummy_token").await?;
    let db = SledRepository::new("./urchin_db")?;

    let notifiers: Vec<Arc<dyn PlatformNotifier>> = vec![Arc::new(discord), Arc::new(telegram)];
    let engine = ConsensusService::new(Arc::new(db), notifiers);

    while let Some(event) = rx.recv().await {
        match event {
            UrchinEvent::RequestAction { kind, target, requester, channel_id, reason } => {
                let _ = engine.request_action(&kind, &target, &requester, &channel_id, &reason).await;
            }
            UrchinEvent::ConfirmAction { target, approver } => {
                let _ = engine.confirm_action(&target, &approver).await;
            }
        }
    }

    Ok(())
}
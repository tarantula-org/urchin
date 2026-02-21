mod domain;
mod infra;

use ::std::sync::Arc;
use ::std::env;
use ::anyhow::{Context, Result};
use ::dotenvy::dotenv;
use ::tokio::sync::mpsc;

use infra::discord::DiscordAdapter;
use infra::stoat::StoatAdapter;
use infra::persistence::SledRepository;
use domain::services::ConsensusService;
use domain::ports::{PlatformNotifier, UrchinEvent};

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv().ok();
    
    let (tx, mut rx) = mpsc::channel::<UrchinEvent>(100);

    let discord_token = env::var("DISCORD_TOKEN").context("DISCORD_TOKEN missing")?;
    let discord_guild_id = env::var("DISCORD_GUILD_ID")?.parse::<u64>()?;
    let discord_staff_role = env::var("DISCORD_STAFF_ROLE_ID")?.parse::<u64>()?;
    let discord_log_channel = env::var("DISCORD_LOG_CHANNEL_ID")?.parse::<u64>()?;
    
    let discord = DiscordAdapter::new(&discord_token, discord_guild_id, discord_staff_role, discord_log_channel, tx.clone()).await?;

    let stoat_token = env::var("STOAT_TOKEN").context("STOAT_TOKEN missing")?;
    let stoat_log_channel = env::var("STOAT_LOG_CHANNEL_ID").context("STOAT_LOG_CHANNEL_ID missing")?;
    let stoat_staff_role = env::var("STOAT_STAFF_ROLE_ID").context("STOAT_STAFF_ROLE_ID missing")?;
    
    let stoat = StoatAdapter::new(&stoat_token, stoat_log_channel, stoat_staff_role, tx).await?;

    let db = SledRepository::new("./urchin_db")?;

    let notifiers: ::std::vec::Vec<Arc<dyn PlatformNotifier>> = vec![
        Arc::new(discord) as Arc<dyn PlatformNotifier>,
        Arc::new(stoat) as Arc<dyn PlatformNotifier>,
    ];

    let engine = ConsensusService::new(Arc::new(db), notifiers);

    while let Some(event) = rx.recv().await {
        match event {
            UrchinEvent::RequestAction { kind, target, requester, origin_platform, origin_channel_id, reason } => {
                let _ = engine.request_action(&kind, &target, &requester, origin_platform, &origin_channel_id, &reason).await;
            }
            UrchinEvent::ConfirmAction { target, approver } => {
                let _ = engine.confirm_action(&target, &approver).await;
            }
        }
    }

    Ok(())
}
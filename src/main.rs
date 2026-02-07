mod domain;
mod infra;

use std::sync::Arc;
use anyhow::Result;
use infra::discord::DiscordAdapter;
use infra::telegram::TelegramAdapter;
use infra::persistence::SledRepository;
use domain::services::ConsensusService;
use domain::ports::PlatformNotifier;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Adapters (Infrastructure)
    let discord = DiscordAdapter::new("TOKEN").await?;
    let telegram = TelegramAdapter::new("TOKEN").await?;
    let db = SledRepository::new("./urchin_db")?;

    // 2. Wiring (Dependency Injection)
    let notifiers: Vec<Arc<dyn PlatformNotifier>> = vec![
        Arc::new(discord),
        Arc::new(telegram),
    ];

    let engine = ConsensusService::new(Arc::new(db), notifiers);

    println!("Urchin Nucleus Active.");

    // 3. Execution (Integration Test)
    let target = "User_123";
    let admin = "Admin_Acrilic";

    engine.request_ban(target, admin).await?;
    engine.check_ban_status(target).await?;

    Ok(())
}
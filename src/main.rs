mod config;
mod domain;
mod infra;

use crate::config::AppConfig;
use crate::domain::{engine::Core, models::Event, ports::Driver};
use crate::infra::{discord::Discord, stoat::Stoat, store::SledStore};
use ::std::{env, sync::Arc};
use ::tokio::sync::mpsc;

#[::tokio::main]
async fn main() -> ::anyhow::Result<()> {
    ::dotenvy::dotenv().ok();
    ::tracing_subscriber::fmt::init();

    let cfg = Arc::new(AppConfig::load("config.toml")?);
    let (tx, mut rx) = mpsc::channel::<Event>(100);
    let store = Arc::new(SledStore::new("./urchin_db")?);

    let discord = Arc::new(Discord::new(
        &env::var("DISCORD_TOKEN")?,
        env::var("DISCORD_GUILD_ID")?.parse()?,
        env::var("DISCORD_STAFF_ROLE_ID")?.parse()?,
        env::var("DISCORD_LOG_CHANNEL_ID")?.parse()?,
        tx.clone(),
        Arc::clone(&cfg)
    ).await?) as Arc<dyn Driver>;

    let stoat = Arc::new(Stoat::new(
        &env::var("STOAT_TOKEN")?,
        &env::var("STOAT_LOG_CHANNEL_ID")?,
        &env::var("STOAT_STAFF_ROLE_ID")?,
        tx.clone(),
        Arc::clone(&cfg)
    ).await?) as Arc<dyn Driver>;

    let core = Core::new(store, vec![discord, stoat], Arc::clone(&cfg));

    let tx_sweep = tx.clone();
    ::tokio::spawn(async move {
        let mut interval = ::tokio::time::interval(::std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            let _ = tx_sweep.send(Event::Sweep).await;
        }
    });

    while let Some(ev) = rx.recv().await {
        if let Err(e) = core.run(ev).await {
            ::tracing::error!("Kernel Exception: {:#}", e);
        }
    }
    
    Ok(())
}
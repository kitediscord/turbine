use std::error::Error;

use futures_util::StreamExt;
use tokio::select;
use twilight_model::id::Id;

use kite_bot::{DiscordCache, DiscordClient, DiscordGateway};
use kite_engine::{PluginEngine, PluginEngineConfig};
use kite_store::MongoDbStoreProvider;

use crate::config::SharedConfig;
use crate::context::ServerContext;

mod auth;
mod config;
mod context;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let config = SharedConfig::read();

    let mongo_store = MongoDbStoreProvider {};
    let discord_client = DiscordClient::new("".to_string(), Id::new(1));
    let discord_cache = DiscordCache::new();

    let engine = PluginEngine::new(PluginEngineConfig {
        discord_client: discord_client.clone(),
        discord_cache: discord_cache.clone(),
        plugin_store: mongo_store.clone().into(),
        deployment_store: mongo_store.clone().into(),
    }).expect("Creating engine");
    engine.spawn_epoch_thread();

    let ctx = ServerContext {
        config,
        engine,
        discord_client,
        discord_cache,
        plugin_store: mongo_store.clone().into(),
        deployment_store: mongo_store.clone().into(),
    };

    let gateway = DiscordGateway::new("".to_string()).await?;
    gateway.spawn();

    select! {
        res = services::serve_services(&ctx) => res?,
        res = consume_gateway(gateway, &ctx) => res,
    }

    Ok(())
}

async fn consume_gateway(mut gateway: DiscordGateway, ctx: &ServerContext) {
    while let Some((_, event)) = gateway.events().next().await {
        ctx.discord_cache.update(&event);
        let engine = ctx.engine.clone();
        tokio::spawn(async move {
            engine.process_discord_event(event).await;
        });
    }
}

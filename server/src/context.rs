use kite_bot::{DiscordCache, DiscordClient};
use kite_engine::PluginEngine;
use kite_store::{AbstractDeploymentStore, AbstractPluginStore};

use crate::SharedConfig;

#[derive(Clone)]
pub struct ServerContext {
    pub config: SharedConfig,
    pub discord_client: DiscordClient,
    pub discord_cache: DiscordCache,

    pub engine: PluginEngine,

    pub plugin_store: AbstractPluginStore,
    pub deployment_store: AbstractDeploymentStore
}
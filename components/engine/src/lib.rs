use std::time::Duration;

use kite_passenger::deployments::v1::DeploymentSettings;
use kite_passenger::events::v1::EventGroup;
use wasmtime::{Config, Engine, Linker, Module, Store};

pub use instance::*;
use kite_bot::{DiscordCache, DiscordClient};
use kite_store::{AbstractDeploymentStore, AbstractPluginStore, PluginModel};
pub use plugin::*;

use crate::limiter::PluginInstanceResourceLimiter;
use crate::linker::create_linker;

mod instance;
mod limiter;
mod linker;
mod plugin;

#[derive(Clone)]
pub struct PluginEngine {
    engine: Engine,
    linker: Linker<PluginInstanceState>,
    config: PluginEngineConfig,
}

impl PluginEngine {
    pub fn new(config: PluginEngineConfig) -> anyhow::Result<Self> {
        let engine = Engine::new(
            &Config::new()
                .async_support(true)
                .epoch_interruption(true)
                .static_memory_forced(true)
                .static_memory_maximum_size(16_000_000),
        )
        .unwrap();

        Ok(Self {
            linker: create_linker(&engine)?,
            engine,
            config,
        })
    }

    pub fn spawn_epoch_thread(&self) {
        let engine = self.engine.clone();
        std::thread::spawn(move || {
            let interval = Duration::from_millis(1);
            loop {
                std::thread::sleep(interval);
                engine.increment_epoch();
            }
        });
    }

    pub fn compile_plugin(&self, model: PluginModel) -> Result<Plugin, anyhow::Error> {
        let module = Module::new(&self.engine, model.module)?;

        Ok(Plugin {
            id: model.id,
            module,
        })
    }

    pub async fn instantiate_plugin(
        &self,
        plugin: &Plugin,
        config: PluginInstanceConfig,
        settings: DeploymentSettings,
    ) -> Result<PluginInstance, anyhow::Error> {
        let mut store = Store::new(
            &self.engine,
            PluginInstanceState {
                group: EventGroup::Unspecified as i32,
                resource_limiter: PluginInstanceResourceLimiter {
                    max_table_size: 0,
                    max_memory_size: config.max_memory_size,
                },
                config,
                settings,
                current_event: None,
                current_action_response: None,
            },
        );
        store.set_epoch_deadline(3);
        store.epoch_deadline_async_yield_and_update(1);
        store.limiter_async(move |s| &mut s.resource_limiter);

        let instance = self
            .linker
            .instantiate_async(&mut store, &plugin.module)
            .await?;

        Ok(PluginInstance::new(instance, store))
    }
}

#[derive(Clone)]
pub struct PluginEngineConfig {
    pub discord_client: DiscordClient,
    pub discord_cache: DiscordCache,
    pub plugin_store: AbstractPluginStore,
    pub deployment_store: AbstractDeploymentStore,
}

use std::sync::Arc;

use dashmap::{DashMap, DashSet};
use twilight_model::gateway::event::Event;
use twilight_model::id::marker::GuildMarker;
use twilight_model::id::Id;

use crate::runtime::PluginRuntime;

mod runtime;

pub struct PluginScheduler {
    deployments: DashMap<Id<GuildMarker>, DashSet<u64>>,
    runtimes: DashMap<(u64, Id<GuildMarker>), PluginRuntime>,
}

impl PluginScheduler {
    async fn get_guild_deployments(&self, _: Id<GuildMarker>) -> Vec<u64> {
        unimplemented!()
    }

    async fn get_guild_runtimes(&self, guild_id: Id<GuildMarker>) -> Vec<PluginRuntime> {
        let _deployments = self.get_guild_deployments(guild_id).await;

        vec![]
    }

    pub async fn process_discord_event(&self, event: Event) {
        let instances = self.get_guild_runtimes(Id::new(1)).await;

        let event = Arc::new(event);
        for instance in instances {
            let event = event.clone();
            tokio::spawn(async move {
                instance.process_discord_event(event).await;
            });
        }
    }
}

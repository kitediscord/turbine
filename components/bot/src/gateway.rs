use std::sync::Arc;

use twilight_gateway::cluster::{ClusterStartError, Events, ShardScheme};
use twilight_gateway::queue::LocalQueue;
use twilight_gateway::Cluster;
use twilight_model::gateway::Intents;

pub struct DiscordGateway {
    cluster: Arc<Cluster>,
    events: Events,
}

impl DiscordGateway {
    pub async fn new(token: String) -> Result<Self, ClusterStartError> {
        let intents = Intents::GUILDS
            | Intents::GUILD_WEBHOOKS
            | Intents::GUILD_MESSAGES
            | Intents::MESSAGE_CONTENT;

        let queue = Arc::new(LocalQueue::new());
        let shard_scheme = ShardScheme::Bucket {
            bucket_id: 0,
            concurrency: 0,
            total: 0,
        };

        let (cluster, events) = Cluster::builder(token, intents)
            .queue(queue)
            .shard_scheme(shard_scheme)
            .build()
            .await?;

        let cluster = Arc::new(cluster);

        Ok(Self { cluster, events })
    }

    pub fn spawn(&self) {
        let cluster = self.cluster.clone();
        tokio::spawn(async move {
            cluster.up().await;
        });
    }

    pub fn events(&mut self) -> &mut Events {
        &mut self.events
    }
}

use std::sync::Arc;
use std::time::Duration;

use kite_passenger::deployments::v1::DeploymentSettings;
use kite_passenger::events::v1::{event, Event, EventGroup, InitEvent};
use tokio::sync::mpsc;
use tokio::time::Instant;
use twilight_model::gateway::event::Event as DiscordEvent;

use kite_engine::{Plugin, PluginEngine, PluginInstance, PluginInstanceConfig};

pub struct PluginRuntime {
    sender: mpsc::Sender<Arc<DiscordEvent>>,
    plugin: Plugin,
}

impl PluginRuntime {
    pub async fn new(engine: PluginEngine, plugin: Plugin) -> anyhow::Result<PluginRuntime> {
        let mut instance = engine
            .instantiate_plugin(
                &plugin,
                PluginInstanceConfig {
                    max_memory_size: 0,
                    cpu_time_per_event: Duration::from_millis(10),
                    execution_time_per_event: Duration::from_secs(5),
                },
                DeploymentSettings {},
            )
            .await?;

        instance
            .run_event_handler(Event {
                group: EventGroup::Init as i32,
                event: Some(event::Event::Init(InitEvent { settings: None })),
            })
            .await?;

        let (sender, receiver) = mpsc::channel(1);
        tokio::spawn(runtime_task(instance, receiver));
        Ok(PluginRuntime { sender, plugin })
    }

    pub fn is_closed(&self) -> bool {
        self.sender.is_closed()
    }

    pub async fn process_discord_event(&self, event: Arc<DiscordEvent>) {
        let _ = self
            .sender
            .send_timeout(event, Duration::from_secs(30))
            .await;
    }
}

async fn runtime_task(
    mut _instance: PluginInstance,
    mut receiver: mpsc::Receiver<Arc<DiscordEvent>>,
) {
    while let Some(_event) = receiver.recv().await {
        // instance.run_event_handler(&Eve).await;
    }
}

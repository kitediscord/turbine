use std::time::Duration;

use anyhow::anyhow;
use future_wrap::WrapFuture;
use kite_passenger::deployments::v1::DeploymentSettings;
use kite_passenger::events::v1::Event;
use prost::Message;
use tokio::time::timeout;
use wasmtime::{Instance, Memory, Store};

use crate::limiter::limit_cpu_time;
use crate::PluginInstanceResourceLimiter;

pub struct PluginInstance {
    instance: Instance,
    store: Store<PluginInstanceState>,
}

impl PluginInstance {
    pub fn new(instance: Instance, store: Store<PluginInstanceState>) -> Self {
        Self { instance, store }
    }

    fn config(&self) -> &PluginInstanceConfig {
        &self.store.data().config
    }

    fn primary_memory(&mut self) -> Result<Memory, anyhow::Error> {
        Ok(self
            .instance
            .get_memory(&mut self.store, "memory")
            .ok_or(anyhow!(""))?)
    }

    fn prepare_event_group(&mut self, group: i32) {
        let state = self.store.data_mut();

        state.group = group;
        state.current_event = None;
        state.current_action_response = None;
    }

    pub async fn run_event_handler(&mut self, event: Event) -> Result<(), anyhow::Error> {
        self.prepare_event_group(event.group);

        let func = self
            .instance
            .get_typed_func::<i32, (), _>(&mut self.store, "_kite_handle_event");

        if let Ok(func) = func {
            let event_buf = event.encode_to_vec();
            let event_size = event_buf.len() as i32;
            self.store.data_mut().current_event = Some(event_buf);

            let mut remaining_cpu_time = self.config().cpu_time_per_event;
            let execution_time = self.config().execution_time_per_event;

            let fut = func
                .call_async(&mut self.store, event_size)
                .wrap(|fut, cx| limit_cpu_time(&mut remaining_cpu_time)(fut, cx));

            timeout(execution_time, fut).await???;
        }

        Ok(())
    }
}

pub struct PluginInstanceConfig {
    pub max_memory_size: usize,
    pub cpu_time_per_event: Duration,
    pub execution_time_per_event: Duration,
}

pub struct PluginInstanceState {
    pub group: i32,
    pub config: PluginInstanceConfig,
    pub resource_limiter: PluginInstanceResourceLimiter,
    pub settings: DeploymentSettings,

    pub current_event: Option<Vec<u8>>,
    pub current_action_response: Option<Vec<u8>>,
}

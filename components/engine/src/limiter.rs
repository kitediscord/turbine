use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use anyhow::anyhow;

use async_trait::async_trait;
use wasmtime::ResourceLimiterAsync;

// This is probably not sufficient to limit memory as it's only called when memory is requested to grow.
// The initial memory size isn't limit by this
// TODO: limit initial memory size

// https://docs.wasmtime.dev/api/wasmtime/struct.InstanceLimits.html
// https://docs.rs/wasmtime/latest/wasmtime/struct.Config.html#method.allocation_strategy

// spawn one task for each deployment and use a channel to send messages to the module instance on that task
// https://github.com/bytecodealliance/wizer

pub struct PluginInstanceResourceLimiter {
    pub max_memory_size: usize,
    pub max_table_size: u32,
}

#[async_trait]
impl ResourceLimiterAsync for PluginInstanceResourceLimiter {
    async fn memory_growing(
        &mut self,
        _current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> bool {
        desired <= self.max_memory_size
    }

    async fn table_growing(&mut self, _current: u32, desired: u32, _maximum: Option<u32>) -> bool {
        desired <= self.max_table_size
    }
}

/// by combining the epoch deadline of wasmtime and this we can pretty accurately limit the cpu time.
/// the epoch deadline will suspend the execution every ~1ms and give us the change to check against the remaining time
pub fn limit_cpu_time<T>(
    remaining: &mut Duration,
) -> impl FnMut(
    Pin<&mut (dyn Future<Output = T> + '_)>,
    &mut Context,
) -> Poll<Result<T, anyhow::Error>>
       + '_ {
    |fut, cx| {
        let poll_start = Instant::now();
        let res = fut.poll(cx);

        *remaining = remaining.saturating_sub(poll_start.elapsed());
        if remaining.is_zero() {
            Poll::Ready(Err(anyhow!("")))
        } else {
            res.map(|v| Ok(v))
        }
    }
}

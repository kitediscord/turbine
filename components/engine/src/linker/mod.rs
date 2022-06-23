use std::future::Future;

use kite_passenger::actions::v1::Action;
use prost::Message;
use wasmtime::{AsContextMut, Caller, Engine, Linker, Memory, StoreContextMut, Trap};

use crate::PluginInstanceState;

pub fn create_linker(engine: &Engine) -> anyhow::Result<Linker<PluginInstanceState>> {
    let mut linker = Linker::new(engine);

    linker.func_wrap1_async("env", "_kite_get_event", host_kite_get_event)?;
    linker.func_wrap2_async("env", "_kite_perform_action", host_kite_perform_action)?;
    linker.func_wrap1_async(
        "env",
        "_kite_get_action_response",
        host_kite_get_action_response,
    )?;

    Ok(linker)
}

fn get_primary_memory(caller: &mut Caller<'_, PluginInstanceState>) -> Result<Memory, Trap> {
    let memory = caller
        .get_export("memory")
        .ok_or(Trap::new("No default 'memory' export"))?
        .into_memory()
        .ok_or(Trap::new("Default 'memory' export is not a Memory"))?;

    Ok(memory)
}

/// This allows us to get the underlying bytes without copying them out with memory.read
fn deref_ptr<'a>(
    memory: &mut Memory,
    ctx: &'a mut StoreContextMut<'_, PluginInstanceState>,
    ptr: i32,
    size: i32,
) -> Result<&'a [u8], Trap> {
    memory
        .data(ctx)
        .get(ptr as usize..(ptr + size) as usize)
        .ok_or(Trap::new("Invalid pointer"))
}

fn host_kite_get_event<'a>(
    mut caller: Caller<'a, PluginInstanceState>,
    ptr: i32,
) -> Box<dyn Future<Output = Result<(), Trap>> + Send + 'a> {
    Box::new(async move {
        let response = caller
            .data_mut()
            .current_event
            .take()
            .ok_or(Trap::new("No event available"))?;

        let memory = get_primary_memory(&mut caller)?;

        memory
            .write(&mut caller.as_context_mut(), ptr as usize, &response)
            .map_err(|_| Trap::new("Invalid pointer"))?;

        Ok(())
    })
}

fn host_kite_perform_action<'a>(
    mut caller: Caller<'a, PluginInstanceState>,
    ptr: i32,
    size: i32,
) -> Box<dyn Future<Output = Result<i32, Trap>> + Send + 'a> {
    Box::new(async move {
        let _: Action = {
            let mut memory = get_primary_memory(&mut caller)?;
            let mut ctx = caller.as_context_mut();
            let buffer = deref_ptr(&mut memory, &mut ctx, ptr, size)?;

            Action::decode(buffer).map_err(|_| Trap::new("Failed to deserialize action"))?
        };

        let resp_buffer = vec![];
        let resp_size = resp_buffer.len() as i32;

        caller.data_mut().current_action_response = Some(resp_buffer);
        Ok(resp_size)
    })
}

fn host_kite_get_action_response<'a>(
    mut caller: Caller<'a, PluginInstanceState>,
    ptr: i32,
) -> Box<dyn Future<Output = Result<(), Trap>> + Send + 'a> {
    Box::new(async move {
        let response = caller
            .data_mut()
            .current_action_response
            .take()
            .ok_or(Trap::new("No action response available"))?;

        let memory = get_primary_memory(&mut caller)?;

        memory
            .write(&mut caller.as_context_mut(), ptr as usize, &response)
            .map_err(|_| Trap::new("Invalid pointer"))?;

        Ok(())
    })
}

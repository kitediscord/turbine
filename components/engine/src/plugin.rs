use wasmtime::Module;

#[derive(Clone)]
pub struct Plugin {
    pub id: u64,
    pub module: Module,
}

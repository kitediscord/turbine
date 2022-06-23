use enum_dispatch::enum_dispatch;

pub use mongodb_provider::*;
pub use traits::*;

mod mongodb_provider;
mod traits;

// using enum_dispatch we can avoid using trait objects and dynamic dispatches

#[enum_dispatch(DeploymentStore)]
#[derive(Clone)]
pub enum AbstractDeploymentStore {
    MongoDbStoreProvider,
}

#[enum_dispatch(PluginStore)]
#[derive(Clone)]
pub enum AbstractPluginStore {
    MongoDbStoreProvider,
}

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

#[async_trait]
#[enum_dispatch]
pub trait PluginStore {
    async fn create_plugin(&self, plugin: &PluginModel);
    async fn update_plugin(&self, plugin: &PluginModel);
    async fn delete_plugin(&self, id: u64);
    async fn get_plugin(&self, id: u64) -> PluginModel;
}

pub struct PluginModel {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub vanity: Option<String>,
    pub module: Vec<u8>
}

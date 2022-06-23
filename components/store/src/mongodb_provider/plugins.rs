use async_trait::async_trait;

use crate::{MongoDbStoreProvider, PluginModel, PluginStore};

#[async_trait]
impl PluginStore for MongoDbStoreProvider {
    async fn create_plugin(&self, _: &PluginModel) {
        todo!()
    }

    async fn update_plugin(&self, _: &PluginModel) {
        todo!()
    }

    async fn delete_plugin(&self, _: u64) {
        todo!()
    }

    async fn get_plugin(&self, _: u64) -> PluginModel {
        todo!()
    }
}

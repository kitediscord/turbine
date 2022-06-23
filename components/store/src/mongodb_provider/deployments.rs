use async_trait::async_trait;

use crate::{DeploymentStore, MongoDbStoreProvider};

#[async_trait]
impl DeploymentStore for MongoDbStoreProvider {}

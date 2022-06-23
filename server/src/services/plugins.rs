use kite_cable::plugins::v1::plugin_service_server::{PluginService};
use kite_cable::plugins::v1::{
    CreatePluginRequest, CreatePluginResponse, CreatePluginVersionRequest,
    CreatePluginVersionResponse, DeletePluginRequest, DeletePluginResponse,
    DeletePluginVersionRequest, DeletePluginVersionResponse, GetPluginRequest, GetPluginResponse,
    ListPluginVersionsRequest, ListPluginVersionsResponse, ListPluginsRequest, ListPluginsResponse,
    Plugin, UpdatePluginRequest, UpdatePluginResponse, UpdatePluginVersionRequest,
    UpdatePluginVersionResponse,
};
use tonic::{Request, Response, Status};

use kite_store::{AbstractPluginStore, PluginStore};

pub struct PluginServicer {
    pub store: AbstractPluginStore,
}

#[tonic::async_trait]
impl PluginService for PluginServicer {
    async fn get_plugin(
        &self,
        req: Request<GetPluginRequest>,
    ) -> Result<Response<GetPluginResponse>, Status> {
        let req = req.into_inner();
        let model = self.store.get_plugin(req.id).await;

        Ok(Response::new(GetPluginResponse {
            plugin: Some(Plugin {
                id: model.id,
                creator_id: 0,
                public: false,
                name: "".to_string(),
                short_description: "".to_string(),
                long_description: "".to_string(),
                tags: vec![],
            }),
        }))
    }

    async fn list_plugins(
        &self,
        _: Request<ListPluginsRequest>,
    ) -> Result<Response<ListPluginsResponse>, Status> {
        todo!()
    }

    async fn create_plugin(
        &self,
        _: Request<CreatePluginRequest>,
    ) -> Result<Response<CreatePluginResponse>, Status> {
        todo!()
    }

    async fn update_plugin(
        &self,
        _: Request<UpdatePluginRequest>,
    ) -> Result<Response<UpdatePluginResponse>, Status> {
        todo!()
    }

    async fn delete_plugin(
        &self,
        _: Request<DeletePluginRequest>,
    ) -> Result<Response<DeletePluginResponse>, Status> {
        todo!()
    }

    async fn list_plugin_versions(
        &self,
        _: Request<ListPluginVersionsRequest>,
    ) -> Result<Response<ListPluginVersionsResponse>, Status> {
        todo!()
    }

    async fn create_plugin_version(
        &self,
        _: Request<CreatePluginVersionRequest>,
    ) -> Result<Response<CreatePluginVersionResponse>, Status> {
        todo!()
    }

    async fn delete_plugin_version(
        &self,
        _: Request<DeletePluginVersionRequest>,
    ) -> Result<Response<DeletePluginVersionResponse>, Status> {
        todo!()
    }

    async fn update_plugin_version(
        &self,
        _: Request<UpdatePluginVersionRequest>,
    ) -> Result<Response<UpdatePluginVersionResponse>, Status> {
        todo!()
    }
}

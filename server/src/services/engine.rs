use std::pin::Pin;

use futures::Stream;
use kite_cable::engine::v1::engine_service_server::{EngineService};
use kite_cable::engine::v1::{
    CreateDeploymentRequest, CreateDeploymentResponse, DeleteDeploymentRequest,
    DeleteDeploymentResponse, GetDeploymentRequest, GetDeploymentResponse, ListDeploymentsRequest,
    ListDeploymentsResponse, StreamDeploymentLogsRequest, StreamDeploymentLogsResponse,
    UpdateDeploymentRequest, UpdateDeploymentResponse,
};
use tonic::{Request, Response, Status};

use kite_engine::PluginEngine;
use kite_store::AbstractDeploymentStore;
use crate::auth::AuthContext;

pub struct EngineServicer {
    pub _store: AbstractDeploymentStore,
    pub _engine: PluginEngine,
}

#[tonic::async_trait]
impl EngineService for EngineServicer {
    async fn create_deployment(
        &self,
        req: Request<CreateDeploymentRequest>,
    ) -> Result<Response<CreateDeploymentResponse>, Status> {
        let _: &AuthContext = req.extensions().get().unwrap();

        todo!()
    }

    async fn update_deployment(
        &self,
        _: Request<UpdateDeploymentRequest>,
    ) -> Result<Response<UpdateDeploymentResponse>, Status> {
        todo!()
    }

    async fn delete_deployment(
        &self,
        _: Request<DeleteDeploymentRequest>,
    ) -> Result<Response<DeleteDeploymentResponse>, Status> {
        todo!()
    }

    async fn list_deployments(
        &self,
        _: Request<ListDeploymentsRequest>,
    ) -> Result<Response<ListDeploymentsResponse>, Status> {
        todo!()
    }

    async fn get_deployment(
        &self,
        _: Request<GetDeploymentRequest>,
    ) -> Result<Response<GetDeploymentResponse>, Status> {
        todo!()
    }

    type StreamDeploymentLogsStream =
        Pin<Box<dyn Stream<Item = Result<StreamDeploymentLogsResponse, Status>> + Send>>;

    async fn stream_deployment_logs(
        &self,
        _: Request<StreamDeploymentLogsRequest>,
    ) -> Result<Response<Self::StreamDeploymentLogsStream>, Status> {
        todo!()
    }
}

use std::net::SocketAddr;

use kite_cable::discord::v1::discord_service_server::DiscordServiceServer;
use kite_cable::engine::v1::engine_service_server::EngineServiceServer;
use kite_cable::plugins::v1::plugin_service_server::PluginServiceServer;
use log::info;
use tonic::transport::Server;
use tonic::{Request, Status};

use crate::auth::decode_auth_token;
use crate::services::discord::DiscordServicer;
use crate::services::engine::EngineServicer;
use crate::services::plugins::PluginServicer;
use crate::ServerContext;

mod discord;
mod engine;
mod plugins;

pub async fn serve_services(ctx: &ServerContext) -> Result<(), tonic::transport::Error> {
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();

    info!("Serving gRPC services on: {}", addr);
    Server::builder()
        .add_service(PluginServiceServer::with_interceptor(
            PluginServicer {
                store: ctx.plugin_store.clone(),
            },
            auth_interceptor,
        ))
        .add_service(EngineServiceServer::with_interceptor(
            EngineServicer {
                _store: ctx.deployment_store.clone(),
                _engine: ctx.engine.clone(),
            },
            auth_interceptor,
        ))
        .add_service(DiscordServiceServer::with_interceptor(
            DiscordServicer {
                discord_cache: ctx.discord_cache.clone(),
            },
            auth_interceptor,
        ))
        .serve(addr)
        .await
}

fn auth_interceptor(mut req: Request<()>) -> Result<Request<()>, Status> {
    match req
        .metadata()
        .get("Authorization")
        .map(|t| t.to_str().ok())
        .flatten()
    {
        Some(token) => match decode_auth_token(token, "").ok() {
            Some(auth) => {
                req.extensions_mut().insert(auth);
                Ok(req)
            }
            None => Err(Status::unauthenticated("Invalid token provided")),
        },
        None => Err(Status::unauthenticated("No token provided")),
    }
}

use kite_cable::discord::v1::discord_service_server::DiscordService;
use kite_cable::discord::v1::{
    GetGuildRequest, GetGuildResponse, Guild, ListGuildsRequest, ListGuildsResponse,
};
use tonic::{Request, Response, Status};
use twilight_model::id::Id;

use kite_bot::DiscordCache;

use crate::auth::AuthContext;

pub struct DiscordServicer {
    pub discord_cache: DiscordCache,
}

#[tonic::async_trait]
impl DiscordService for DiscordServicer {
    async fn list_guilds(
        &self,
        req: Request<ListGuildsRequest>,
    ) -> Result<Response<ListGuildsResponse>, Status> {
        let auth: &AuthContext = req.extensions().get().unwrap();
        if !auth.is_internal() {
            return Err(Status::unauthenticated(
                "Not available for external services",
            ));
        }

        let req = req.into_inner();
        let guilds = req
            .ids
            .into_iter()
            .filter_map(|gid| {
                Id::new_checked(gid)
                    .map(|gid| {
                        self.discord_cache.guild(gid).map(|g| Guild {
                            id: g.id().get(),
                            name: g.name().to_string(),
                        })
                    })
                    .flatten()
            })
            .collect();

        Ok(Response::new(ListGuildsResponse { guilds }))
    }

    async fn get_guild(
        &self,
        req: Request<GetGuildRequest>,
    ) -> Result<Response<GetGuildResponse>, Status> {
        let auth: &AuthContext = req.extensions().get().unwrap();
        if !auth.is_internal() {
            return Err(Status::unauthenticated(
                "Not available for external services",
            ));
        }

        let req = req.into_inner();
        if req.id == 0 {
            return Err(Status::invalid_argument("Id can't be 0"));
        }

        let guild = self.discord_cache.guild(Id::new(req.id)).map(|g| Guild {
            id: g.id().get(),
            name: g.name().to_string(),
        });

        match guild {
            Some(guild) => Ok(Response::new(GetGuildResponse { guild: Some(guild) })),
            None => Err(Status::not_found("Unknown guild")),
        }
    }
}

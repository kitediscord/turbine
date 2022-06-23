use std::ops::Deref;
use std::sync::Arc;

use twilight_http::client::InteractionClient;
use twilight_http::Client;
use twilight_model::id::marker::ApplicationMarker;
use twilight_model::id::Id;

struct DiscordClientInner {
    client: Client,
    application_id: Id<ApplicationMarker>,
}

#[derive(Clone)]
pub struct DiscordClient(Arc<DiscordClientInner>);

impl DiscordClient {
    pub fn new(token: String, application_id: Id<ApplicationMarker>) -> Self {
        Self(Arc::new(DiscordClientInner {
            client: Client::new(token),
            application_id,
        }))
    }

    pub fn interaction_client(&self) -> InteractionClient {
        self.0.client.interaction(self.0.application_id)
    }
}

impl Deref for DiscordClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0.client
    }
}

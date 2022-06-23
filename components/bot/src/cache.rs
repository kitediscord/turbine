use std::ops::Deref;
use std::sync::Arc;

use twilight_cache_inmemory::{InMemoryCache, ResourceType};

#[derive(Clone)]
pub struct DiscordCache(Arc<InMemoryCache>);

impl DiscordCache {
    pub fn new() -> Self {
        Self(Arc::new(
            InMemoryCache::builder()
                .resource_types(
                    ResourceType::ROLE
                        | ResourceType::CHANNEL
                        | ResourceType::GUILD
                        | ResourceType::EMOJI
                        | ResourceType::STICKER,
                )
                .build(),
        ))
    }
}

impl Deref for DiscordCache {
    type Target = InMemoryCache;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

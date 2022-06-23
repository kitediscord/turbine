use std::sync::Arc;

use config::Config;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct SharedConfig(Arc<RootConfig>);

impl SharedConfig {
    pub fn read() -> Self {
        Self(Arc::new(RootConfig::read()))
    }
}

#[derive(Deserialize, Debug)]
pub struct RootConfig {}

impl RootConfig {
    pub fn read() -> Self {
        let config = Config::builder()
            .add_source(config::File::with_name("Turbine"))
            .add_source(config::Environment::with_prefix("TURBINE"))
            .build()
            .unwrap();

        config.try_deserialize().expect("Parsing config")
    }
}

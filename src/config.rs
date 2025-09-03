use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct MobiusConfig {
    #[serde(default = "default_port")]
    pub listen_port: &'static str,
    #[serde(default = "prefix")]
    pub prefix: &'static str,
    #[serde(default = "shared")]
    pub shared: bool,
}

impl MobiusConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(Environment::with_prefix("MOBIUS"))
            .build()?;

        config.try_deserialize()
    }
}

fn default_port() -> &'static str {
    "8554"
}

fn prefix() -> &'static str {
    "mobius-stream"
}

fn shared() -> bool {
    true
}

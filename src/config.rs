use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct MobiusConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "prefix")]
    pub prefix: &'static str,
    #[serde(default = "shared")]
    pub shared: bool,
    #[serde(default = "infinite")]
    pub infinite: bool
}

impl MobiusConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(Environment::with_prefix("MOBIUS"))
            .build()?;

        config.try_deserialize()
    }
}

fn default_port() -> u16 {
    8554
}

fn prefix() -> &'static str {
    "mobius-stream"
}

fn shared() -> bool {
    true
}

fn infinite() -> bool {
    true
}
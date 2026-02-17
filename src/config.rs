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
    pub infinite: bool,
    #[serde(default = "corrupted")]
    pub corrupted: bool,
    #[serde(default = "min_disconnected_time")]
    pub min_disconnected_time: u64,
    #[serde(default = "max_disconnected_time")]
    pub max_disconnected_time: u64,
    #[serde(default = "min_time_between_break")]
    pub min_time_between_break: u64,
    #[serde(default = "max_time_between_break")]
    pub max_time_between_break: u64,
    #[serde(default = "min_drop_probability")]
    pub min_drop_probability: f32,
    #[serde(default = "max_drop_probability")]
    pub max_drop_probability: f32,
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

fn corrupted() -> bool {
    false
}

fn min_disconnected_time() -> u64 {
    10
}

fn max_disconnected_time() -> u64 {
    30
}

fn min_time_between_break() -> u64 {
    10
}

fn max_time_between_break() -> u64 {
    30
}

fn min_drop_probability() -> f32 {
    0.05
}

fn max_drop_probability() -> f32 {
    0.9
}

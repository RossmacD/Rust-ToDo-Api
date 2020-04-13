use serde::Deserialize;
use config::ConfigError;

#[derive(Deserialize)]
pub struct ServerConfig{
    pub url: String,
    pub port: i32
}

#[derive(Deserialize)]
pub struct Config{
    pub server: ServerConfig
}

impl Config{
    pub fn from_env() -> Result<Self, ConfigError>{
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        return cfg.try_into();
    }
}
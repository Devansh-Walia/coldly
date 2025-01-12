use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub port: u16,
    pub smtp_email: String,
    pub smtp_password: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(".env").required(false))
            .add_source(Environment::default())
            .build()?;
        
        config.try_deserialize()
    }
}
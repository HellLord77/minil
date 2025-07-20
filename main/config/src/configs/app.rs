use config::Config;
use config::ConfigError;
use config::Environment;
use serde::Deserialize;
use serde::Serialize;

use crate::configs::DatabaseConfig;
use crate::configs::LogConfig;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {
    pub log: LogConfig,

    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn try_new() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(Config::try_from(&Self::default())?)
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?
            .try_deserialize()
    }

    pub fn new() -> Self {
        Self::try_new().unwrap_or_default()
    }
}

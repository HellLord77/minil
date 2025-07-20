use serde::Deserialize;
use serde::Serialize;
use smart_default::SmartDefault;

use crate::types::LogLevel;

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
pub struct DatabaseConfig {
    #[default = "sqlite::memory:"]
    pub url: String,

    #[default(LogLevel::Debug)]
    pub log_level: LogLevel,

    #[default(LogLevel::Warn)]
    pub slow_log_level: LogLevel,

    #[default = 1]
    pub slow_threshold: u64,
}

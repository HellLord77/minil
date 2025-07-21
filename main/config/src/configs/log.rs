use serde::Deserialize;
use serde::Serialize;
use smart_default::SmartDefault;

use crate::types::LogFormat;
use crate::types::LogLevel;
use crate::types::LogStream;

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
pub struct LogConfig {
    #[cfg_attr(debug_assertions, default(LogLevel::Debug))]
    #[cfg_attr(not(debug_assertions), default(LogLevel::Warn))]
    pub level: LogLevel,

    pub stream: LogStream,

    #[cfg_attr(debug_assertions, default(LogFormat::Pretty))]
    #[cfg_attr(not(debug_assertions), default(LogFormat::Compact))]
    pub format: LogFormat,
}

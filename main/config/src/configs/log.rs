use serde::Deserialize;
use serde::Serialize;

use crate::types::LogFormat;
use crate::types::LogLevel;
use crate::types::LogStream;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: LogLevel,

    pub stream: LogStream,

    pub format: LogFormat,
}

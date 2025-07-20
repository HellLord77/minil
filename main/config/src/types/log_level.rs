use serde::Deserialize;
use serde::Serialize;
use tracing::Level;
use tracing::log::LevelFilter;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Off,
    Error,
    #[cfg_attr(not(debug_assertions), default)]
    Warn,
    Info,
    #[cfg_attr(debug_assertions, default)]
    Debug,
    Trace,
}

impl LogLevel {
    pub fn as_filter(&self) -> LevelFilter {
        match self {
            Self::Off => LevelFilter::Off,
            Self::Error => LevelFilter::Error,
            Self::Warn => LevelFilter::Warn,
            Self::Info => LevelFilter::Info,
            Self::Debug => LevelFilter::Debug,
            Self::Trace => LevelFilter::Trace,
        }
    }

    pub fn try_as_level(&self) -> Option<Level> {
        match self {
            Self::Off => None,
            Self::Error => Some(Level::ERROR),
            Self::Warn => Some(Level::WARN),
            Self::Info => Some(Level::INFO),
            Self::Debug => Some(Level::DEBUG),
            Self::Trace => Some(Level::TRACE),
        }
    }
}

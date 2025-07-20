use serde::Deserialize;
use serde::Serialize;
use tracing::Level;
use tracing::Subscriber;
use tracing::log::LevelFilter;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::registry::LookupSpan;

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

    fn try_as_level(&self) -> Option<Level> {
        match self {
            Self::Off => None,
            Self::Error => Some(Level::ERROR),
            Self::Warn => Some(Level::WARN),
            Self::Info => Some(Level::INFO),
            Self::Debug => Some(Level::DEBUG),
            Self::Trace => Some(Level::TRACE),
        }
    }

    pub fn to_layer<S>(&self, target: &str) -> Box<dyn Layer<S> + Send + Sync + 'static>
    where
        S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    {
        match EnvFilter::try_from_default_env() {
            Ok(filter) => filter.boxed(),
            Err(_) => Targets::new()
                .with_target(target, self.try_as_level())
                .boxed(),
        }
    }
}

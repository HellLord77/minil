use std::io;

use serde::Deserialize;
use serde::Serialize;
use tracing_appender::non_blocking::NonBlocking;
use tracing_appender::non_blocking::WorkerGuard;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogStream {
    #[default]
    StdOut,
    StdErr,
}

impl LogStream {
    pub fn to_writer(&self) -> (NonBlocking, WorkerGuard) {
        match self {
            Self::StdOut => tracing_appender::non_blocking(io::stdout()),
            Self::StdErr => tracing_appender::non_blocking(io::stderr()),
        }
    }
}

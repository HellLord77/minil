use serde::Deserialize;
use serde::Serialize;

use crate::utils::FormatBehavior;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat {
    #[default]
    Full,
    Compact,
    Pretty,
    Json,
}

impl LogFormat {
    pub fn to_format(&self) -> FormatBehavior {
        match self {
            Self::Full => FormatBehavior::full(),
            Self::Compact => FormatBehavior::compact(),
            Self::Pretty => FormatBehavior::pretty(),
            Self::Json => FormatBehavior::json(),
        }
    }
}

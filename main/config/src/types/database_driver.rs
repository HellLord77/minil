use std::fmt;
use std::fmt::Display;

use fmt::Formatter;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseDriver {
    #[default]
    Sqlite,
    Postgres,
    Mysql,
}

impl Display for DatabaseDriver {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite => write!(f, "sqlite"),
            Self::Postgres => write!(f, "postgres"),
            Self::Mysql => write!(f, "mysql"),
        }
    }
}

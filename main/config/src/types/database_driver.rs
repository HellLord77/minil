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

impl DatabaseDriver {
    pub fn try_as_port(&self) -> Option<u16> {
        match self {
            Self::Sqlite => None,
            Self::Postgres => Some(5432),
            Self::Mysql => Some(3306),
        }
    }
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

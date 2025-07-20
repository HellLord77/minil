use serde::Deserialize;
use serde::Serialize;
use smart_default::SmartDefault;
use url::Url;

use crate::error::UrlParseError;
use crate::error::UrlParseResult;
use crate::types::DatabaseDriver;
use crate::types::LogLevel;

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub driver: DatabaseDriver,

    #[default = ":memory:"]
    pub host: String,

    pub port: Option<u16>,

    pub username: String,

    pub password: Option<String>,

    pub database: String,

    pub params: Option<String>,

    pub url: Option<Url>,

    #[default(LogLevel::Debug)]
    pub log_level: LogLevel,

    #[default(LogLevel::Warn)]
    pub slow_log_level: LogLevel,

    #[default = 1]
    pub slow_threshold: u64,
}

impl DatabaseConfig {
    pub fn try_to_url(&self) -> UrlParseResult {
        Ok(match &self.url {
            Some(url) => url.clone(),
            None => match self.driver {
                DatabaseDriver::Sqlite if self.host == ":memory:" => {
                    Url::parse("sqlite::memory:").unwrap_or_else(|_err| unreachable!())
                }
                _ => {
                    let mut url =
                        Url::parse("sqlite:///:memory:").unwrap_or_else(|_err| unreachable!());

                    url.set_scheme(&self.driver.to_string())
                        .map_err(|_| UrlParseError::Scheme)?;
                    url.set_host(Some(if self.host == ":memory:" {
                        "localhost"
                    } else {
                        &self.host
                    }))?;
                    url.set_port(self.port.or_else(|| self.driver.try_as_port()))
                        .map_err(|_| UrlParseError::Port)?;
                    url.set_username(&self.username)
                        .map_err(|_| UrlParseError::Username)?;
                    url.set_password(self.password.as_deref())
                        .map_err(|_| UrlParseError::Password)?;
                    url.set_path(&self.database);
                    url.set_query(self.params.as_deref());

                    url
                }
            },
        })
    }
}

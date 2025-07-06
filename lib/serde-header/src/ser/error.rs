use std::fmt::Display;
use std::str::Utf8Error;

use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use serde::de::Unexpected;
use serde::ser;

#[derive(Debug, PartialEq, Display, From, Error)]
pub enum Error {
    #[display("{}", _0)]
    Custom(#[error(not(source))] String),
    Utf8(Utf8Error),
}

impl Error {
    pub fn unsupported<T>(unexp: Unexpected) -> Result<T, Self> {
        Err(Self::Custom(format!("{unexp} is not supported")))
    }

    pub fn unsupported_header<T>(unexp: Unexpected) -> Result<T, Self> {
        Err(Self::Custom(format!("{unexp} is not supported header")))
    }

    pub fn unsupported_name<T>(unexp: Unexpected) -> Result<T, Self> {
        Err(Self::Custom(format!("{unexp} is not supported name")))
    }

    pub fn unsupported_value<T>(unexp: Unexpected) -> Result<T, Self> {
        Err(Self::Custom(format!("{unexp} is not supported value")))
    }

    pub fn header_done() -> Self {
        Self::Custom("header has already been serialized".to_owned())
    }

    pub fn header_not_done() -> Self {
        Self::Custom("header has not yet been serialized".to_owned())
    }

    pub fn map_no_name() -> Self {
        Self::Custom("map name has not yet been serialized".to_owned())
    }

    pub fn map_no_value() -> Self {
        Self::Custom("map value has not yet been serialized".to_owned())
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}

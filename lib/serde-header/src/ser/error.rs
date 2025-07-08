use std::fmt::Display;
use std::str::Utf8Error;

use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use serde::de::Unexpected;
use serde::ser;

#[derive(Debug, PartialEq, Display, From, Error)]
pub enum Error {
    Custom(#[error(not(source))] String),
    Utf8(Utf8Error),
}

impl Error {
    pub fn unsupported<T>(unexp: Unexpected) -> Result<T, Self> {
        Err(format!("{unexp} is not supported"))?
    }

    pub fn unsupported_header<T>(unexp: Unexpected) -> Result<T, Self> {
        Err(format!("{unexp} is not supported header"))?
    }

    pub fn unsupported_name<T>(unexp: Unexpected) -> Result<T, Self> {
        Err(format!("{unexp} is not supported name"))?
    }

    pub fn unsupported_value<T>(unexp: Unexpected) -> Result<T, Self> {
        Err(format!("{unexp} is not supported value"))?
    }

    pub fn map_no_name() -> Self {
        "map name has not yet been serialized".to_owned().into()
    }

    pub fn map_no_value() -> Self {
        "map value has not yet been serialized".to_owned().into()
    }

    pub fn header_done() -> Self {
        "header has already been serialized".to_owned().into()
    }

    pub fn header_not_done() -> Self {
        "header has not yet been serialized".to_owned().into()
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        msg.to_string().into()
    }
}

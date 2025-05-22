use crate::renamer::Renamer;
use crate::str::Str;
use std::error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::result;
use strum::IntoStaticStr;
use strum::VariantNames;

#[cfg(feature = "convert_case")]
use crate::convert_case::ConvertCase;

#[cfg(feature = "heck")]
use crate::heck::Heck;

#[cfg(feature = "ident_case")]
use crate::ident_case::IdentCase;

#[cfg(feature = "inflector")]
use crate::inflector::Inflector;

#[derive(Debug, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum ValueErrorKind {
    Str,

    #[cfg(feature = "ident_case")]
    IdentCase,

    #[cfg(feature = "convert_case")]
    ConvertCase,

    #[cfg(feature = "heck")]
    Heck,

    #[cfg(feature = "inflector")]
    Inflector,
}

impl ValueErrorKind {
    pub(crate) fn get_variants(&self) -> &'static [&'static str] {
        match self {
            ValueErrorKind::Str => Str::VARIANTS,

            #[cfg(feature = "ident_case")]
            ValueErrorKind::IdentCase => IdentCase::VARIANTS,

            #[cfg(feature = "convert_case")]
            ValueErrorKind::ConvertCase => ConvertCase::VARIANTS,

            #[cfg(feature = "heck")]
            ValueErrorKind::Heck => Heck::VARIANTS,

            #[cfg(feature = "inflector")]
            ValueErrorKind::Inflector => Inflector::VARIANTS,
        }
    }
}

#[derive(Debug)]
pub(crate) enum Error<'a> {
    Name(&'a str),
    Value(&'a str, ValueErrorKind),
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (kind, unknown, variants) = match self {
            Error::Name(unknown) => ("renamer", unknown, Renamer::VARIANTS),
            Error::Value(unknown, kind) => (kind.into(), unknown, kind.get_variants()),
        };
        write!(
            f,
            "unknown {kind} `{unknown}`, expected one of: {}",
            variants.join(", ")
        )
    }
}

impl error::Error for Error<'_> {}

pub(crate) type Result<'a, T> = result::Result<T, Error<'a>>;

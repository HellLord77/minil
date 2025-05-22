use crate::renamer::Renamer;
use crate::str::Str;
use derive_more::Constructor;
use strum::Display;
use strum::EnumIs;
use strum::VariantNames;
use thiserror::Error;

#[cfg(feature = "convert_case")]
use crate::convert_case::ConvertCase;

#[cfg(feature = "heck")]
use crate::heck::Heck;

#[cfg(feature = "ident_case")]
use crate::ident_case::IdentCase;

#[cfg(feature = "inflector")]
use crate::inflector::Inflector;

#[derive(Debug, Display, EnumIs)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum TryNewErrorKind {
    Renamer,
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

impl TryNewErrorKind {
    pub(crate) fn variants(&self) -> &'static [&'static str] {
        match self {
            TryNewErrorKind::Renamer => Renamer::VARIANTS,
            Self::Str => Str::VARIANTS,

            #[cfg(feature = "ident_case")]
            Self::IdentCase => IdentCase::VARIANTS,

            #[cfg(feature = "convert_case")]
            Self::ConvertCase => ConvertCase::VARIANTS,

            #[cfg(feature = "heck")]
            ValueErrorKind::Heck => Heck::VARIANTS,

            #[cfg(feature = "inflector")]
            Self::Inflector => Inflector::VARIANTS,
        }
    }
}

#[derive(Debug, Constructor, Error)]
#[error("unknown renamer `{unknown}`, expected one of {expected}", expected = self.kind.variants().join(", "))]
pub(crate) struct TryNewError {
    unknown: String,
    kind: TryNewErrorKind,
}

impl TryNewError {
    #[inline]
    pub(crate) fn from_renamer(unknown: String) -> Self {
        Self::new(unknown, TryNewErrorKind::Renamer)
    }

    #[inline]
    pub(crate) fn kind(&self) -> &TryNewErrorKind {
        &self.kind
    }
}

pub(crate) type TryNewResult<T> = Result<T, TryNewError>;

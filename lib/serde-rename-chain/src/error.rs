use derive_more::Constructor;
use derive_more::Display;
use derive_more::Error;
use derive_more::IsVariant;
use strum::VariantNames;

use crate::renamer::Renamer;
use crate::str::Str;

#[rustfmt::skip]
#[cfg(feature = "convert_case")]
use crate::convert_case::ConvertCase;

#[rustfmt::skip]
#[cfg(feature = "heck")]
use crate::heck::Heck;

#[rustfmt::skip]
#[cfg(feature = "ident_case")]
use crate::ident_case::IdentCase;

#[rustfmt::skip]
#[cfg(feature = "inflector")]
use crate::inflector::Inflector;

#[derive(Debug, IsVariant)]
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
    #[inline]
    pub(crate) fn get_variants(&self) -> &'static [&'static str] {
        match self {
            Self::Renamer => Renamer::VARIANTS,
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

#[derive(Debug, Display, Constructor, Error)]
#[display("unknown renamer `{unknown}`, expected one of {}", kind.get_variants().join(", "))]
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
    pub(crate) fn is_renamer(&self) -> bool {
        self.kind.is_renamer()
    }
}

pub(crate) type TryNewResult<T> = Result<T, TryNewError>;

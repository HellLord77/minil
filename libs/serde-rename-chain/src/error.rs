use crate::renamer::Renamer;
use std::fmt::{Display, Formatter, Result};
use strum::VariantNames;

#[cfg(feature = "convert_case")]
use crate::convert_case::ConvertCase;
#[cfg(feature = "heck")]
use crate::heck::Heck;
#[cfg(feature = "ident_case")]
use crate::ident_case::IdentCase;
#[cfg(feature = "inflector")]
use crate::inflector::Inflector;

pub(crate) enum RenamerError<'a> {
    Name(&'a str),
    Value(ValueError<'a>),
}

impl Display for RenamerError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let message = match self {
            RenamerError::Name(unknown) => {
                format!(
                    "unknown renamer `{unknown}`, expected one of {}",
                    Renamer::VARIANTS.join(", ")
                )
            }
            RenamerError::Value(err) => err.to_string(),
        };
        write!(f, "{}", message)
    }
}

pub(crate) enum ValueError<'a> {
    #[cfg(feature = "ident_case")]
    IdentCase(&'a str),
    #[cfg(feature = "convert_case")]
    ConvertCase(&'a str),
    #[cfg(feature = "heck")]
    Heck(&'a str),
    #[cfg(feature = "inflector")]
    Inflector(&'a str),
}

impl Display for ValueError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (name, unknown, variants) = match self {
            #[cfg(feature = "ident_case")]
            ValueError::IdentCase(unknown) => ("ident_case", unknown, IdentCase::VARIANTS),
            #[cfg(feature = "convert_case")]
            ValueError::ConvertCase(unknown) => ("convert_case", unknown, ConvertCase::VARIANTS),
            #[cfg(feature = "heck")]
            ValueError::Heck(unknown) => ("heck", unknown, Heck::VARIANTS),
            #[cfg(feature = "inflector")]
            ValueError::Inflector(unknown) => ("inflector", unknown, Inflector::VARIANTS),
        };
        write!(
            f,
            "unknown {name} `{unknown}`, expected one of: {}",
            variants.join(", ")
        )
    }
}

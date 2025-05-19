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

pub(crate) enum RenamerError {
    Name(String),
    Value(ValueError),
}

impl Display for RenamerError {
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

pub(crate) enum ValueError {
    #[cfg(feature = "ident_case")]
    IdentCase(String),
    #[cfg(feature = "convert_case")]
    ConvertCase(String),
    #[cfg(feature = "heck")]
    Heck(String),
    #[cfg(feature = "inflector")]
    Inflector(String),
}

impl Display for ValueError {
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

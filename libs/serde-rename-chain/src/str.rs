use crate::error::ValueError;
use std::str::FromStr;
use strum::EnumString;
use strum::VariantNames;

#[derive(EnumString, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Str {
    Lower,
    Upper,
    AsciiUpper,
    AsciiLower,
}

impl Str {
    pub(crate) fn try_from_str(s: &str) -> crate::Result<Self> {
        Self::from_str(s).map_err(|_err| crate::Error::Value(ValueError::Str(s)))
    }

    pub(crate) fn apply(&self, s: &str) -> String {
        let str = match self {
            Self::Lower => str::to_lowercase,
            Self::Upper => str::to_uppercase,
            Self::AsciiUpper => str::to_ascii_uppercase,
            Self::AsciiLower => str::to_ascii_lowercase,
        };
        str(s)
    }
}

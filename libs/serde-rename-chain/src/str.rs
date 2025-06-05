use strum::EnumString;
use strum::VariantNames;

use crate::error::TryNewErrorKind;
use crate::renamer::TryNewValue;

#[derive(Debug, EnumString, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Str {
    Lower,
    Upper,
    AsciiUpper,
    AsciiLower,
}

impl Str {
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

impl TryNewValue for Str {
    const KIND: TryNewErrorKind = TryNewErrorKind::Str;
}

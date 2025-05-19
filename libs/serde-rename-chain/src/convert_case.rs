use crate::error::{RenamerError, ValueError};
use convert_case::{Case, Casing};
use std::str::FromStr;
use strum::{EnumString, VariantNames};

#[derive(EnumString, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum ConvertCase {
    Snake,
    Constant,
    UpperSnake,
    Ada,
    Kebab,
    Cobol,
    UpperKebab,
    Train,
    Flat,
    UpperFlat,
    Pascal,
    UpperCamel,
    Camel,
    Lower,
    Upper,
    Title,
    Sentence,
    Alternating,
    Toggle,
    #[cfg(feature = "convert_case_random")]
    Random,
    #[cfg(feature = "convert_case_random")]
    PseudoRandom,
}

impl ConvertCase {
    pub(crate) fn try_from_str(s: &str) -> Result<Self, RenamerError> {
        Self::from_str(s).map_err(|_| RenamerError::Value(ValueError::ConvertCase(s.to_owned())))
    }

    pub(crate) fn apply(&self, s: &str) -> String {
        let convert_case = match self {
            Self::Snake => Case::Snake,
            Self::Constant => Case::Constant,
            Self::UpperSnake => Case::UpperSnake,
            Self::Ada => Case::Ada,
            Self::Kebab => Case::Kebab,
            Self::Cobol => Case::Cobol,
            Self::UpperKebab => Case::UpperKebab,
            Self::Train => Case::Train,
            Self::Flat => Case::Flat,
            Self::UpperFlat => Case::UpperFlat,
            Self::Pascal => Case::Pascal,
            Self::UpperCamel => Case::UpperCamel,
            Self::Camel => Case::Camel,
            Self::Lower => Case::Lower,
            Self::Upper => Case::Upper,
            Self::Title => Case::Title,
            Self::Sentence => Case::Sentence,
            Self::Alternating => Case::Alternating,
            Self::Toggle => Case::Toggle,
            #[cfg(feature = "convert_case_random")]
            Self::Random => Case::Random,
            #[cfg(feature = "convert_case_random")]
            Self::PseudoRandom => Case::PseudoRandom,
        };
        s.to_case(convert_case)
    }
}

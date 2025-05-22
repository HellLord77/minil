use crate::error::ValueErrorKind;
use crate::renamer::TryNewValue;
use convert_case::Case;
use convert_case::Casing;
use strum::EnumString;
use strum::VariantNames;

#[derive(Debug, EnumString, VariantNames)]
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

impl TryNewValue for ConvertCase {
    const KIND: ValueErrorKind = ValueErrorKind::ConvertCase;
}

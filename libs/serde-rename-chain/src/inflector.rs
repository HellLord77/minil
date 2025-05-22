use crate::error::TryNewErrorKind;
use crate::renamer::TryNewValue;
use inflector::Inflector as InflectorTrait;
use strum::EnumString;
use strum::VariantNames;

#[derive(Debug, EnumString, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Inflector {
    Camel,
    Pascal,
    Snake,
    ScreamingSnake,
    Kebab,
    Train,
    Sentence,
    Title,
    ForeignKey,

    #[cfg(feature = "inflector_heavyweight")]
    Class,
    #[cfg(feature = "inflector_heavyweight")]
    Table,
    #[cfg(feature = "inflector_heavyweight")]
    Plural,
    #[cfg(feature = "inflector_heavyweight")]
    Singular,
}

impl Inflector {
    pub(crate) fn apply(&self, s: &str) -> String {
        let inflector = match self {
            Self::Camel => str::to_camel_case,
            Self::Pascal => str::to_pascal_case,
            Self::Snake => str::to_snake_case,
            Self::ScreamingSnake => str::to_screaming_snake_case,
            Self::Kebab => str::to_kebab_case,
            Self::Train => str::to_train_case,
            Self::Sentence => str::to_sentence_case,
            Self::Title => str::to_title_case,
            Self::ForeignKey => str::to_foreign_key,

            #[cfg(feature = "inflector_heavyweight")]
            Self::Class => str::to_class_case,
            #[cfg(feature = "inflector_heavyweight")]
            Self::Table => str::to_table_case,
            #[cfg(feature = "inflector_heavyweight")]
            Self::Plural => str::to_plural,
            #[cfg(feature = "inflector_heavyweight")]
            Self::Singular => str::to_singular,
        };

        inflector(s)
    }
}

impl TryNewValue for Inflector {
    const KIND: TryNewErrorKind = TryNewErrorKind::Inflector;
}

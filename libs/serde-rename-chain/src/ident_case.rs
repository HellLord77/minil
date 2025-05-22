use crate::error::ValueErrorKind;
use crate::renamer::TryNewValue;
use ident_case::RenameRule;
use strum::EnumString;
use strum::VariantNames;

#[derive(Debug, EnumString, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum IdentCase {
    None,
    Lower,
    Pascal,
    Camel,
    Snake,
    ScreamingSnake,
    Kebab,
}

impl IdentCase {
    pub(crate) fn apply(&self, s: &str) -> String {
        let ident_case = match self {
            Self::None => RenameRule::None,
            Self::Lower => RenameRule::LowerCase,
            Self::Pascal => RenameRule::PascalCase,
            Self::Camel => RenameRule::CamelCase,
            Self::Snake => RenameRule::SnakeCase,
            Self::ScreamingSnake => RenameRule::ScreamingSnakeCase,
            Self::Kebab => RenameRule::KebabCase,
        };

        ident_case.apply_to_variant(s)
    }
}

impl TryNewValue for IdentCase {
    const KIND: ValueErrorKind = ValueErrorKind::IdentCase;
}

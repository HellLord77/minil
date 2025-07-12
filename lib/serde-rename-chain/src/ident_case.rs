use ident_case::RenameRule;
use strum::EnumString;
use strum::VariantNames;

use crate::error::TryNewErrorKind;
use crate::renamer::TryNewValue;

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
    fn get_rename_rule(&self) -> RenameRule {
        match self {
            Self::None => RenameRule::None,
            Self::Lower => RenameRule::LowerCase,
            Self::Pascal => RenameRule::PascalCase,
            Self::Camel => RenameRule::CamelCase,
            Self::Snake => RenameRule::SnakeCase,
            Self::ScreamingSnake => RenameRule::ScreamingSnakeCase,
            Self::Kebab => RenameRule::KebabCase,
        }
    }

    pub(crate) fn apply_enum(&self, s: &str) -> String {
        let ident_case = self.get_rename_rule();

        ident_case.apply_to_variant(s)
    }

    pub(crate) fn apply_struct(&self, s: &str) -> String {
        let ident_case = self.get_rename_rule();

        ident_case.apply_to_field(s)
    }
}

impl TryNewValue for IdentCase {
    const KIND: TryNewErrorKind = TryNewErrorKind::IdentCase;
}

use crate::error::ValueError;
use ident_case::RenameRule;
use std::str::FromStr;
use strum::{EnumString, VariantNames};

#[derive(EnumString, VariantNames)]
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
    pub(crate) fn try_from_str(s: &str) -> crate::Result<Self> {
        Self::from_str(s).map_err(|_| crate::Error::Value(ValueError::IdentCase(s)))
    }

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

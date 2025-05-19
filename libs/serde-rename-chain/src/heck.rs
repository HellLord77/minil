use crate::{error::RenamerError, error::ValueError};
use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToTitleCase, ToTrainCase, ToUpperCamelCase,
};
use ident_case::RenameRule;
use std::str::FromStr;
use strum::{EnumString, VariantNames};

#[derive(EnumString, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Heck {
    Kebab,
    LowerCamel,
    ShoutyKebab,
    ShoutySnake,
    ShoutySnek,
    Snake,
    Snek,
    Title,
    Train,
    UpperCamel,
    Pascal,
}

impl Heck {
    pub(crate) fn try_from_str(s: &str) -> Result<Self, RenamerError> {
        Self::from_str(s).map_err(|_| RenamerError::Value(ValueError::IdentCase))
    }

    pub(crate) fn apply(&self, s: &str) -> String {
        let heck = match self {
            Self::Kebab => str::to_kebab_case,
            Self::LowerCamel => str::to_lower_camel_case,
            Self::ShoutyKebab => str::to_shouty_kebab_case,
            Self::ShoutySnake => str::to_shouty_snake_case,
            Self::ShoutySnek => str::TO_SHOUTY_SNEK_CASE,
            Self::Snake => str::to_snake_case,
            Self::Snek => str::to_snek_case,
            Self::Title => str::to_title_case,
            Self::Train => str::to_train_case,
            Self::UpperCamel => str::to_upper_camel_case,
            Self::Pascal => str::to_pascal_case,
        };
        heck(s)
    }
}

#[cfg(feature = "convert_case")]
use convert_case::{Case, Casing};
#[cfg(feature = "heck")]
use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
    ToTrainCase, ToUpperCamelCase,
};
#[cfg(feature = "inflector")]
use inflector::Inflector;

pub(crate) enum Renamer {
    AddPrefix(String),
    AddSuffix(String),
    StripPrefix(String),
    StripSuffix(String),
    #[cfg(feature = "convert_case")]
    ConvertCase(Case<'static>),
    #[cfg(feature = "heck")]
    Heck(fn(&str) -> String),
    #[cfg(feature = "inflector")]
    Inflector(fn(&str) -> String),
}

impl Renamer {
    pub(crate) fn try_from_arg(name: &str, value: &str) -> Result<Self, RenamerError> {
        let renamer = match name {
            "add_prefix" => Renamer::AddPrefix(value.to_owned()),
            "add_suffix" => Renamer::AddSuffix(value.to_owned()),
            "strip_prefix" => Renamer::StripPrefix(value.to_owned()),
            "strip_suffix" => Renamer::StripSuffix(value.to_owned()),
            #[cfg(feature = "convert_case")]
            "convert_case" => {
                let convert_case = match value {
                    "snake" => Case::Snake,
                    "constant" => Case::Constant,
                    "upper_snake" => Case::UpperSnake,
                    "ada" => Case::Ada,
                    "kebab" => Case::Kebab,
                    "cobol" => Case::Cobol,
                    "upper_kebab" => Case::UpperKebab,
                    "train" => Case::Train,
                    "flat" => Case::Flat,
                    "upper_flat" => Case::UpperFlat,
                    "pascal" => Case::Pascal,
                    "upper_camel" => Case::UpperCamel,
                    "lower" => Case::Lower,
                    "upper" => Case::Upper,
                    "title" => Case::Title,
                    "sentence" => Case::Sentence,
                    "alternating" => Case::Alternating,
                    "toggle" => Case::Toggle,
                    #[cfg(feature = "convert_case_random")]
                    "random" => Case::Random,
                    #[cfg(feature = "convert_case_random")]
                    "pseudo_random" => Case::PseudoRandom,
                    _ => return Err(RenamerError::ConvertCase),
                };
                Renamer::ConvertCase(convert_case)
            }
            #[cfg(feature = "heck")]
            "heck" => {
                let heck = match value {
                    "kebab" => str::to_kebab_case,
                    "lower_camel" => str::to_lower_camel_case,
                    "shouty_kebab" => str::to_shouty_kebab_case,
                    "shouty_snake" | "shouty_snek" => str::to_shouty_snake_case,
                    "snake" | "snek" => str::to_snake_case,
                    "title" => str::to_title_case,
                    "train" => str::to_train_case,
                    "upper_camel" | "pascal" => str::to_upper_camel_case,
                    _ => return Err(RenamerError::Heck),
                };
                Renamer::Heck(heck)
            }
            #[cfg(feature = "inflector")]
            "inflector" => {
                let inflector = match value {
                    "camel" => str::to_camel_case,
                    "pascal" => str::to_pascal_case,
                    "snake" => str::to_snake_case,
                    "screaming_snake" => str::to_screaming_snake_case,
                    "kebab" => str::to_kebab_case,
                    "train" => str::to_train_case,
                    "sentence" => str::to_sentence_case,
                    "title" => str::to_title_case,
                    "foreign_key" => str::to_foreign_key,
                    #[cfg(feature = "inflector_heavyweight")]
                    "class" => str::to_class_case,
                    #[cfg(feature = "inflector_heavyweight")]
                    "table" => str::to_table_case,
                    #[cfg(feature = "inflector_heavyweight")]
                    "plural" => str::to_plural,
                    #[cfg(feature = "inflector_heavyweight")]
                    "singular" => str::to_singular,
                    _ => return Err(RenamerError::Inflector),
                };
                Renamer::Inflector(inflector)
            }
            _ => return Err(RenamerError::Name),
        };
        Ok(renamer)
    }

    pub(crate) fn apply(&self, name: &str) -> String {
        match self {
            Renamer::AddPrefix(prefix) => format!("{}{}", prefix, name),
            Renamer::AddSuffix(suffix) => format!("{}{}", name, suffix),
            Renamer::StripPrefix(prefix) => name.strip_prefix(prefix).unwrap_or(name).to_owned(),
            Renamer::StripSuffix(suffix) => name.strip_suffix(suffix).unwrap_or(name).to_owned(),
            #[cfg(feature = "convert_case")]
            Renamer::ConvertCase(convert_case) => name.to_case(*convert_case),
            #[cfg(feature = "heck")]
            Renamer::Heck(heck) => heck(name),
            #[cfg(feature = "inflector")]
            Renamer::Inflector(inflector) => inflector(name),
        }
    }
}

pub(crate) enum RenamerError {
    Name,
    #[cfg(feature = "convert_case")]
    ConvertCase,
    #[cfg(feature = "heck")]
    Heck,
    #[cfg(feature = "inflector")]
    Inflector,
}

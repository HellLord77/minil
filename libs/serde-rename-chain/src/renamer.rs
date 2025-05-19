use crate::error::RenamerError;
use strum::VariantNames;

#[cfg(feature = "convert_case")]
use crate::convert_case::ConvertCase;
#[cfg(feature = "heck")]
use crate::heck::Heck;
#[cfg(feature = "ident_case")]
use crate::ident_case::IdentCase;
#[cfg(feature = "inflector")]
use crate::inflector::Inflector;

#[derive(VariantNames)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Renamer {
    AddPrefix(String),
    AddSuffix(String),
    StripPrefix(String),
    StripSuffix(String),
    #[cfg(feature = "ident_case")]
    IdentCase(IdentCase),
    #[cfg(feature = "convert_case")]
    ConvertCase(ConvertCase),
    #[cfg(feature = "heck")]
    Heck(Heck),
    #[cfg(feature = "inflector")]
    Inflector(Inflector),
}

impl Renamer {
    pub(crate) fn try_from_arg<'a>(n: &'a str, v: &'a str) -> Result<Self, RenamerError<'a>> {
        let renamer = match n {
            "add_prefix" => Renamer::AddPrefix(v.to_owned()),
            "add_suffix" => Renamer::AddSuffix(v.to_owned()),
            "strip_prefix" => Renamer::StripPrefix(v.to_owned()),
            "strip_suffix" => Renamer::StripSuffix(v.to_owned()),
            #[cfg(feature = "ident_case")]
            "ident_case" => Renamer::IdentCase(IdentCase::try_from_str(v)?),
            #[cfg(feature = "convert_case")]
            "convert_case" => Renamer::ConvertCase(ConvertCase::try_from_str(v)?),
            #[cfg(feature = "heck")]
            "heck" => Renamer::Heck(Heck::try_from_str(v)?),
            #[cfg(feature = "inflector")]
            "inflector" => Renamer::Inflector(Inflector::try_from_str(v)?),
            _ => return Err(RenamerError::Name(n)),
        };
        Ok(renamer)
    }

    pub(crate) fn apply(&self, s: &str) -> String {
        match self {
            Renamer::AddPrefix(prefix) => format!("{prefix}{s}"),
            Renamer::AddSuffix(suffix) => format!("{s}{suffix}"),
            Renamer::StripPrefix(prefix) => s.strip_prefix(prefix).unwrap_or(s).to_owned(),
            Renamer::StripSuffix(suffix) => s.strip_suffix(suffix).unwrap_or(s).to_owned(),
            #[cfg(feature = "ident_case")]
            Renamer::IdentCase(ident_case) => ident_case.apply(s),
            #[cfg(feature = "convert_case")]
            Renamer::ConvertCase(convert_case) => convert_case.apply(s),
            #[cfg(feature = "heck")]
            Renamer::Heck(heck) => heck.apply(s),
            #[cfg(feature = "inflector")]
            Renamer::Inflector(inflector) => inflector.apply(s),
        }
    }
}

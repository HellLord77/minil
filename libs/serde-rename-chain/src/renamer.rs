use crate::error::TryNewError;
use crate::error::TryNewErrorKind;
use crate::error::TryNewResult;
use crate::str::Str;
use derive_more::From;
use std::str::FromStr;
use strum::EnumDiscriminants;
use strum::EnumString;
use strum::VariantNames;

#[cfg(feature = "convert_case")]
use crate::convert_case::ConvertCase;

#[cfg(feature = "heck")]
use crate::heck::Heck;

#[cfg(feature = "ident_case")]
use crate::ident_case::IdentCase;

#[cfg(feature = "inflector")]
use crate::inflector::Inflector;

#[derive(Debug, From, VariantNames, EnumDiscriminants)]
#[strum(serialize_all = "snake_case")]
#[strum_discriminants(derive(EnumString), strum(serialize_all = "snake_case"))]
pub(crate) enum Renamer {
    #[from(skip)]
    AddPrefix(String),

    #[from(skip)]
    AddSuffix(String),

    #[from(skip)]
    StripPrefix(String),

    #[from(skip)]
    StripSuffix(String),

    Str(Str),

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
    pub(crate) fn apply(&self, s: &str) -> String {
        match self {
            Renamer::AddPrefix(prefix) => format!("{prefix}{s}"),
            Renamer::AddSuffix(suffix) => format!("{s}{suffix}"),
            Renamer::StripPrefix(prefix) => s.strip_prefix(prefix).unwrap_or(s).to_owned(),
            Renamer::StripSuffix(suffix) => s.strip_suffix(suffix).unwrap_or(s).to_owned(),
            Renamer::Str(str) => str.apply(s),

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

impl TryFrom<(String, String)> for Renamer {
    type Error = TryNewError;

    fn try_from((n, v): (String, String)) -> TryNewResult<Self> {
        Ok(match RenamerDiscriminants::try_new(n)? {
            RenamerDiscriminants::AddPrefix => Renamer::AddPrefix(v),
            RenamerDiscriminants::AddSuffix => Renamer::AddSuffix(v),
            RenamerDiscriminants::StripPrefix => Renamer::StripPrefix(v),
            RenamerDiscriminants::StripSuffix => Renamer::StripSuffix(v),
            RenamerDiscriminants::Str => Str::try_new(v)?.into(),

            #[cfg(feature = "ident_case")]
            RenamerDiscriminants::IdentCase => IdentCase::try_new(v)?.into(),

            #[cfg(feature = "convert_case")]
            RenamerDiscriminants::ConvertCase => ConvertCase::try_new(v)?.into(),

            #[cfg(feature = "heck")]
            RenamerDiscriminants::Heck => Heck::try_new(v)?.into(),

            #[cfg(feature = "inflector")]
            RenamerDiscriminants::Inflector => Inflector::try_new(v)?.into(),
        })
    }
}

impl RenamerDiscriminants {
    #[inline]
    pub(crate) fn try_new(s: String) -> TryNewResult<Self> {
        s.parse().map_err(|_err| TryNewError::from_renamer(s))
    }
}

pub(crate) trait TryNewValue: FromStr {
    const KIND: TryNewErrorKind;

    #[inline]
    fn try_new(s: String) -> TryNewResult<Self> {
        s.parse().map_err(|_err| TryNewError::new(s, Self::KIND))
    }
}

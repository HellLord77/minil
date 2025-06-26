use std::borrow::Cow;
use std::str::FromStr;

use strum::EnumDiscriminants;
use strum::EnumString;
use strum::VariantNames;

use crate::error::TryNewError;
use crate::error::TryNewErrorKind;
use crate::error::TryNewResult;
use crate::str::Str;

#[rustfmt::skip]
#[cfg(feature = "convert_case")]
use crate::convert_case::ConvertCase;

#[rustfmt::skip]
#[cfg(feature = "heck")]
use crate::heck::Heck;

#[rustfmt::skip]
#[cfg(feature = "ident_case")]
use crate::ident_case::IdentCase;

#[rustfmt::skip]
#[cfg(feature = "inflector")]
use crate::inflector::Inflector;

#[rustfmt::skip]
#[cfg(feature = "strfmt")]
use strfmt::strfmt;

#[rustfmt::skip]
#[cfg(feature = "_dynfmt")]
use dynfmt::Format;

#[rustfmt::skip]
#[cfg(feature = "dynfmt_python")]
use dynfmt::PythonFormat;

#[rustfmt::skip]
#[cfg(feature = "dynfmt_curly")]
use dynfmt::SimpleCurlyFormat;

#[derive(Debug, VariantNames, EnumDiscriminants)]
#[strum(serialize_all = "snake_case")]
#[strum_discriminants(derive(EnumString), strum(serialize_all = "snake_case"))]
pub(crate) enum Renamer {
    AddPrefix(String),

    AddSuffix(String),

    StripPrefix(String),

    StripSuffix(String),

    TrimStart(String),

    TrimEnd(String),

    Str(Str),

    #[cfg(feature = "ident_case")]
    IdentCase(IdentCase),

    #[cfg(feature = "convert_case")]
    ConvertCase(ConvertCase),

    #[cfg(feature = "heck")]
    Heck(Heck),

    #[cfg(feature = "inflector")]
    Inflector(Inflector),

    #[cfg(feature = "strfmt")]
    StrFmt(String),

    #[cfg(feature = "dynfmt_python")]
    DynFmtPython(String),

    #[cfg(feature = "dynfmt_curly")]
    DynFmtCurly(String),
}

impl Renamer {
    pub(crate) fn apply(&self, s: &str) -> String {
        match self {
            Self::AddPrefix(prefix) => format!("{prefix}{s}"),
            Self::AddSuffix(suffix) => format!("{s}{suffix}"),
            Self::StripPrefix(prefix) => s.strip_prefix(prefix).unwrap_or(s).to_owned(),
            Self::StripSuffix(suffix) => s.strip_suffix(suffix).unwrap_or(s).to_owned(),
            Self::TrimStart(pattern) => s.trim_start_matches(pattern).to_owned(),
            Self::TrimEnd(pattern) => s.trim_end_matches(pattern).to_owned(),
            Self::Str(str) => str.apply(s),

            #[cfg(feature = "ident_case")]
            Self::IdentCase(ident_case) => ident_case.apply(s),

            #[cfg(feature = "convert_case")]
            Self::ConvertCase(convert_case) => convert_case.apply(s),

            #[cfg(feature = "heck")]
            Self::Heck(heck) => heck.apply(s),

            #[cfg(feature = "inflector")]
            Self::Inflector(inflector) => inflector.apply(s),

            #[cfg(feature = "strfmt")]
            Self::StrFmt(fmt) => {
                strfmt(fmt, &Self::vars(s).into()).unwrap_or_else(|_err| s.to_owned())
            }

            #[cfg(feature = "dynfmt_python")]
            Self::DynFmtPython(fmt) => PythonFormat
                .format(fmt, Self::vars(s).map(|(_, v)| v))
                .map_or_else(|_err| s.to_owned(), Cow::into_owned),

            #[cfg(feature = "dynfmt_curly")]
            Self::DynFmtCurly(fmt) => SimpleCurlyFormat
                .format(fmt, Self::vars(s).map(|(_, v)| v))
                .map_or_else(|_err| s.to_owned(), Cow::into_owned),
        }
    }

    fn vars(s: &str) -> [(char, String); 3] {
        [
            ('s', s.to_owned()),
            ('l', s.len().to_string()),
            ('c', s.chars().count().to_string()),
        ]
    }
}

impl TryFrom<(String, String)> for Renamer {
    type Error = TryNewError;

    #[inline]
    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        Ok(match RenamerDiscriminants::try_new(value.0)? {
            RenamerDiscriminants::AddPrefix => Self::AddPrefix(value.1),
            RenamerDiscriminants::AddSuffix => Self::AddSuffix(value.1),
            RenamerDiscriminants::StripPrefix => Self::StripPrefix(value.1),
            RenamerDiscriminants::StripSuffix => Self::StripSuffix(value.1),
            RenamerDiscriminants::TrimStart => Self::TrimStart(value.1),
            RenamerDiscriminants::TrimEnd => Self::TrimEnd(value.1),
            RenamerDiscriminants::Str => Self::Str(Str::try_new(value.1)?),

            #[cfg(feature = "ident_case")]
            RenamerDiscriminants::IdentCase => Self::IdentCase(IdentCase::try_new(value.1)?),

            #[cfg(feature = "convert_case")]
            RenamerDiscriminants::ConvertCase => Self::ConvertCase(ConvertCase::try_new(value.1)?),

            #[cfg(feature = "heck")]
            RenamerDiscriminants::Heck => Self::Heck(Heck::try_new(value.1)?),

            #[cfg(feature = "inflector")]
            RenamerDiscriminants::Inflector => Self::Inflector(Inflector::try_new(value.1)?),

            #[cfg(feature = "strfmt")]
            RenamerDiscriminants::StrFmt => Self::StrFmt(value.1),

            #[cfg(feature = "dynfmt_python")]
            RenamerDiscriminants::DynFmtPython => Self::DynFmtPython(value.1),

            #[cfg(feature = "dynfmt_curly")]
            RenamerDiscriminants::DynFmtCurly => Self::DynFmtCurly(value.1),
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

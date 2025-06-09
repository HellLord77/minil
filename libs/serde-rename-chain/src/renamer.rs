use std::str::FromStr;

use derive_more::From;
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

    #[from(skip)]
    TrimStart(String),

    #[from(skip)]
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

    #[from(skip)]
    #[cfg(feature = "strfmt")]
    StrFmt(String),
}

impl Renamer {
    pub(crate) fn apply(&self, s: &str) -> String {
        match self {
            Renamer::AddPrefix(prefix) => format!("{prefix}{s}"),
            Renamer::AddSuffix(suffix) => format!("{s}{suffix}"),
            Renamer::StripPrefix(prefix) => s.strip_prefix(prefix).unwrap_or(s).to_owned(),
            Renamer::StripSuffix(suffix) => s.strip_suffix(suffix).unwrap_or(s).to_owned(),
            Renamer::TrimStart(pattern) => s.trim_start_matches(pattern).to_owned(),
            Renamer::TrimEnd(pattern) => s.trim_end_matches(pattern).to_owned(),
            Renamer::Str(str) => str.apply(s),

            #[cfg(feature = "ident_case")]
            Renamer::IdentCase(ident_case) => ident_case.apply(s),

            #[cfg(feature = "convert_case")]
            Renamer::ConvertCase(convert_case) => convert_case.apply(s),

            #[cfg(feature = "heck")]
            Renamer::Heck(heck) => heck.apply(s),

            #[cfg(feature = "inflector")]
            Renamer::Inflector(inflector) => inflector.apply(s),

            #[cfg(feature = "strfmt")]
            Renamer::StrFmt(fmt) => strfmt(
                fmt,
                &[
                    ('s', s.to_owned()),
                    ('l', s.len().to_string()),
                    ('c', s.chars().count().to_string()),
                ]
                .into(),
            )
            .unwrap_or(s.to_owned()),
        }
    }
}

pub(crate) trait TryIntoRenamer {
    type Error;

    fn try_into_renamer(self) -> Result<Renamer, Self::Error>;
}

impl TryIntoRenamer for (String, String) {
    type Error = TryNewError;

    #[inline]
    fn try_into_renamer(self) -> Result<Renamer, Self::Error> {
        Ok(match RenamerDiscriminants::try_new(self.0)? {
            RenamerDiscriminants::AddPrefix => Renamer::AddPrefix(self.1),
            RenamerDiscriminants::AddSuffix => Renamer::AddSuffix(self.1),
            RenamerDiscriminants::StripPrefix => Renamer::StripPrefix(self.1),
            RenamerDiscriminants::StripSuffix => Renamer::StripSuffix(self.1),
            RenamerDiscriminants::TrimStart => Renamer::TrimStart(self.1),
            RenamerDiscriminants::TrimEnd => Renamer::TrimEnd(self.1),
            RenamerDiscriminants::Str => Str::try_new(self.1)?.into(),

            #[cfg(feature = "ident_case")]
            RenamerDiscriminants::IdentCase => IdentCase::try_new(self.1)?.into(),

            #[cfg(feature = "convert_case")]
            RenamerDiscriminants::ConvertCase => ConvertCase::try_new(self.1)?.into(),

            #[cfg(feature = "heck")]
            RenamerDiscriminants::Heck => Heck::try_new(self.1)?.into(),

            #[cfg(feature = "inflector")]
            RenamerDiscriminants::Inflector => Inflector::try_new(self.1)?.into(),

            #[cfg(feature = "strfmt")]
            RenamerDiscriminants::StrFmt => Renamer::StrFmt(self.1),
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

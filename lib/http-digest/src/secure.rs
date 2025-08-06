use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;
use std::str::FromStr;

use base64::prelude::*;
use derive_more::From;
use strum::Display;
use strum::EnumDiscriminants;
use strum::EnumString;
use strum::IntoDiscriminant;

use crate::DigestParseError;
use crate::ValueParseError;

pub type Sha256 = [u8; 32];
pub type Sha512 = [u8; 64];

#[derive(Debug, From, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, Display), strum(serialize_all = "lowercase"))]
pub enum SecureDigest {
    #[strum_discriminants(strum(serialize = "sha-256"))]
    Sha256(Sha256),

    #[strum_discriminants(strum(serialize = "sha-512"))]
    Sha512(Sha512),
}

impl Deref for SecureDigest {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            SecureDigest::Sha256(d) => d,
            SecureDigest::Sha512(d) => d,
        }
    }
}

impl FromStr for SecureDigest {
    type Err = DigestParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, v) = s.split_once("=").ok_or_else(|| s.to_owned())?;
        let a = a.to_lowercase().parse()?;
        let v = v
            .strip_prefix(":")
            .ok_or_else(|| ValueParseError::PrefixColonNotFound(s.to_owned()))?;
        let v = v
            .strip_suffix(":")
            .ok_or_else(|| ValueParseError::SuffixColonNotFound(s.to_owned()))?;
        let v = BASE64_STANDARD
            .decode(v.as_bytes())
            .map_err(ValueParseError::from)?;

        Ok(match a {
            SecureDigestDiscriminants::Sha256 => {
                Sha256::try_from(v).map_err(ValueParseError::from)?.into()
            }
            SecureDigestDiscriminants::Sha512 => {
                Sha512::try_from(v).map_err(ValueParseError::from)?.into()
            }
        })
    }
}

impl Display for SecureDigest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}=:{}:",
            self.discriminant(),
            BASE64_STANDARD.encode(self.deref())
        )
    }
}

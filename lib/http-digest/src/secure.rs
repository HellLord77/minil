use std::str::FromStr;

use base64::prelude::*;
use derive_more::Display;
use derive_more::From;
use strum::EnumDiscriminants;
use strum::EnumString;
use strum::IntoDiscriminant;

use crate::DigestParseError;
use crate::ValueParseError;
use crate::macros::define_digest_algorithm;

define_digest_algorithm!(Sha256, 32);
define_digest_algorithm!(Sha512, 64);

#[derive(Debug, Display, From, EnumDiscriminants)]
#[display("{}=:{}:", self.discriminant(), BASE64_STANDARD.encode(_0.0))]
#[strum_discriminants(derive(EnumString, strum::Display), strum(serialize_all = "lowercase"))]
pub enum SecureDigest {
    #[strum_discriminants(strum(serialize = "sha-256"))]
    Sha256(Sha256),

    #[strum_discriminants(strum(serialize = "sha-512"))]
    Sha512(Sha512),
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
            SecureDigestDiscriminants::Sha256 => Sha256::try_from(v)?.into(),
            SecureDigestDiscriminants::Sha512 => Sha512::try_from(v)?.into(),
        })
    }
}

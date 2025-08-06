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

define_digest_algorithm!(DigestSha256, 32);
define_digest_algorithm!(DigestSha512, 64);

#[derive(Debug, Display, From, EnumDiscriminants)]
#[display("{}=:{}:", self.discriminant(), BASE64_STANDARD.encode(_0.0))]
#[strum_discriminants(derive(EnumString, strum::Display), strum(serialize_all = "lowercase"))]
pub enum SecureDigest {
    #[strum_discriminants(strum(serialize = "sha-256"))]
    Sha256(DigestSha256),

    #[strum_discriminants(strum(serialize = "sha-512"))]
    Sha512(DigestSha512),
}

impl FromStr for SecureDigest {
    type Err = DigestParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, v) = s.split_once('=').ok_or_else(|| s.to_owned())?;
        let a = a.to_lowercase().parse()?;
        if !v.starts_with(':') {
            Err(ValueParseError::PrefixColonNotFound(v.to_owned()))?;
        }
        if !v.ends_with(':') {
            Err(ValueParseError::SuffixColonNotFound(v.to_owned()))?;
        }

        Ok(match a {
            SecureDigestDiscriminants::Sha256 => v.parse::<DigestSha256>()?.into(),
            SecureDigestDiscriminants::Sha512 => v.parse::<DigestSha512>()?.into(),
        })
    }
}

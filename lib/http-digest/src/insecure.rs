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

define_digest_algorithm!(Md5, 16);
define_digest_algorithm!(Sha, 20);
define_digest_algorithm!(UnixSum, 16);
define_digest_algorithm!(UnixCkSum, 32);
define_digest_algorithm!(Adler, 32);
define_digest_algorithm!(Crc32C, 4);

#[derive(Debug, Display, From, EnumDiscriminants)]
#[display("{}=:{}:", self.discriminant(), BASE64_STANDARD.encode(_0.0))]
#[strum_discriminants(derive(EnumString, strum::Display), strum(serialize_all = "lowercase"))]
pub enum InsecureDigest {
    Md5(DigestMd5),

    Sha(DigestSha),

    UnixSum(DigestUnixSum),

    UnixCkSum(DigestUnixCkSum),

    Adler(DigestAdler),

    Crc32C(DigestCrc32C),
}

impl FromStr for InsecureDigest {
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
            InsecureDigestDiscriminants::Md5 => v.parse::<DigestMd5>()?.into(),
            InsecureDigestDiscriminants::Sha => v.parse::<DigestSha>()?.into(),
            InsecureDigestDiscriminants::UnixSum => v.parse::<DigestUnixSum>()?.into(),
            InsecureDigestDiscriminants::UnixCkSum => v.parse::<DigestUnixCkSum>()?.into(),
            InsecureDigestDiscriminants::Adler => v.parse::<DigestAdler>()?.into(),
            InsecureDigestDiscriminants::Crc32C => v.parse::<DigestCrc32C>()?.into(),
        })
    }
}

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
    Md5(Md5),

    Sha(Sha),

    #[from(skip)]
    UnixSum(UnixSum),

    UnixCkSum(UnixCkSum),

    #[from(skip)]
    Adler(Adler),

    Crc32C(Crc32C),
}

impl FromStr for InsecureDigest {
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
            InsecureDigestDiscriminants::Md5 => Md5::try_from(v)?.into(),
            InsecureDigestDiscriminants::Sha => Sha::try_from(v)?.into(),
            InsecureDigestDiscriminants::UnixSum => Self::UnixSum(UnixSum::try_from(v)?),
            InsecureDigestDiscriminants::UnixCkSum => UnixCkSum::try_from(v)?.into(),
            InsecureDigestDiscriminants::Adler => Self::Adler(Adler::try_from(v)?),
            InsecureDigestDiscriminants::Crc32C => Crc32C::try_from(v)?.into(),
        })
    }
}

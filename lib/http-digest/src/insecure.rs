use std::ops::Deref;
use std::str::FromStr;

use base64::prelude::*;
use derive_more::Display;
use derive_more::From;
use strum::EnumDiscriminants;
use strum::EnumString;
use strum::IntoDiscriminant;

use crate::DigestParseError;
use crate::ValueParseError;

pub type Md5 = [u8; 16];
pub type Sha = [u8; 20];
pub type UnixSum = [u8; 16];
pub type UnixCkSum = [u8; 32];
pub type Adler = [u8; 32];
pub type Crc32C = [u8; 4];

#[derive(Debug, Display, From, EnumDiscriminants)]
#[display("{}=:{}:", self.discriminant(), BASE64_STANDARD.encode(_0))]
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

impl Deref for InsecureDigest {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            InsecureDigest::Md5(d) => d,
            InsecureDigest::Sha(d) => d,
            InsecureDigest::UnixSum(d) => d,
            InsecureDigest::UnixCkSum(d) => d,
            InsecureDigest::Adler(d) => d,
            InsecureDigest::Crc32C(d) => d,
        }
    }
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
            InsecureDigestDiscriminants::Md5 => {
                Md5::try_from(v).map_err(ValueParseError::from)?.into()
            }
            InsecureDigestDiscriminants::Sha => {
                Sha::try_from(v).map_err(ValueParseError::from)?.into()
            }
            InsecureDigestDiscriminants::UnixSum => {
                Self::UnixSum(UnixSum::try_from(v).map_err(ValueParseError::from)?)
            }
            InsecureDigestDiscriminants::UnixCkSum => UnixCkSum::try_from(v)
                .map_err(ValueParseError::from)?
                .into(),
            InsecureDigestDiscriminants::Adler => {
                Self::Adler(Adler::try_from(v).map_err(ValueParseError::from)?)
            }
            InsecureDigestDiscriminants::Crc32C => {
                Crc32C::try_from(v).map_err(ValueParseError::from)?.into()
            }
        })
    }
}

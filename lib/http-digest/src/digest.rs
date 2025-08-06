use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;
use std::str::FromStr;

use crate::DigestParseError;
use crate::InsecureDigest;
use crate::SecureDigest;

#[derive(Debug)]
pub enum Digest {
    Secure(SecureDigest),
    Insecure(InsecureDigest),
}

impl Deref for Digest {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            Digest::Secure(d) => d,
            Digest::Insecure(d) => d,
        }
    }
}

impl FromStr for Digest {
    type Err = DigestParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(d) => Ok(Digest::Secure(d)),
            Err(DigestParseError::InvalidAlgorithm(_)) => s.parse().map(Digest::Insecure),
            Err(e) => Err(e),
        }
    }
}

impl Display for Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Digest::Secure(d) => d.to_string(),
            Digest::Insecure(d) => d.to_string(),
        };

        write!(f, "{s}")
    }
}

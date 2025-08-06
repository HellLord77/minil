use std::str::FromStr;

use derive_more::Display;

use crate::DigestParseError;
use crate::InsecureDigest;
use crate::SecureDigest;

#[derive(Debug, Display)]
pub enum Digest {
    Secure(SecureDigest),
    Insecure(InsecureDigest),
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

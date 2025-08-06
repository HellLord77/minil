mod digest;
mod error;
mod insecure;
mod secure;

pub use digest::Digest;
pub use error::DigestParseError;
pub use error::ValueParseError;
pub use insecure::InsecureDigest;
pub use secure::SecureDigest;

pub type Digests = Vec<Digest>;
pub type DigestsRef<'a> = &'a [Digest];

pub type SecureDigests = Vec<SecureDigest>;
pub type SecureDigestsRef<'a> = &'a [SecureDigest];

pub fn from_str(s: &str) -> Result<Digests, DigestParseError> {
    s.split(",")
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<_>, _>>()
}

pub fn from_str_secure(s: &str) -> Result<SecureDigests, DigestParseError> {
    s.split(",")
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<_>, _>>()
}

pub fn to_string(digests: DigestsRef) -> String {
    digests
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn to_string_secure(digests: SecureDigestsRef) -> String {
    digests
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

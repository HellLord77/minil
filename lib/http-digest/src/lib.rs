mod digest;
mod error;
mod insecure;
mod macros;
mod secure;

pub use digest::Digest;
pub use error::DigestParseError;
pub use error::ValueParseError;
pub use insecure::DigestAdler;
pub use insecure::DigestCrc32C;
pub use insecure::DigestMd5;
pub use insecure::DigestSha;
pub use insecure::DigestUnixCkSum;
pub use insecure::DigestUnixSum;
pub use insecure::InsecureDigest;
pub use secure::DigestSha256;
pub use secure::DigestSha512;
pub use secure::SecureDigest;

pub fn from_str(s: &str) -> Result<Vec<SecureDigest>, DigestParseError> {
    s.split(',').map(|s| s.trim().parse()).collect()
}

pub fn from_str_legacy(s: &str) -> Result<Vec<Digest>, DigestParseError> {
    s.split(',').map(|s| s.trim().parse()).collect()
}

pub fn to_string(digests: &[SecureDigest]) -> String {
    digests
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn to_string_legacy(digests: &[Digest]) -> String {
    digests
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

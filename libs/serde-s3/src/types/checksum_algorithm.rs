use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChecksumAlgorithm {
    Crc32,
    Crc32C,
    Sha1,
    Sha256,
    Crc64Nvme,
}

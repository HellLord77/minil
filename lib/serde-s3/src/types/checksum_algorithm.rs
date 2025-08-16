use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChecksumAlgorithm {
    #[serde(rename = "CRC32")]
    Crc32,
    #[serde(rename = "CRC32C")]
    Crc32C,
    #[serde(rename = "MD5")]
    Md5,
    #[serde(rename = "SHA1")]
    Sha1,
    #[serde(rename = "SHA256")]
    Sha256,
    #[serde(rename = "CRC64NVME")]
    Crc64Nvme,
}

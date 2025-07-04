use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChecksumAlgorithm {
    Crc32,
    Crc32C,
    Crc64Nvme,
    Sha1,
    Sha256,
}

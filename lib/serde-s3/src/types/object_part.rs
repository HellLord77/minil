use bon::Builder;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Builder, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPart {
    pub part_number: Option<u16>,

    pub size: Option<u64>,

    #[serde(rename = "ChecksumCRC32")]
    pub checksum_crc32: Option<String>,

    #[serde(rename = "ChecksumCRC32C")]
    pub checksum_crc32_c: Option<String>,

    #[serde(rename = "ChecksumCRC64NVME")]
    pub checksum_crc64_nvme: Option<String>,

    #[serde(rename = "ChecksumSHA1")]
    pub checksum_sha1: Option<String>,

    #[serde(rename = "ChecksumSHA256")]
    pub checksum_sha256: Option<String>,
}

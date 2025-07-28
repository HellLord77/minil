use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Part {
    #[serde(rename = "ChecksumCRC32")]
    pub checksum_crc32: Option<String>,

    #[serde(rename = "ChecksumCRC32C")]
    pub checksum_crc32c: Option<String>,

    #[serde(rename = "ChecksumCRC64NVME")]
    pub checksum_crc64nvme: Option<String>,

    #[serde(rename = "ChecksumSHA1")]
    pub checksum_sha1: Option<String>,

    #[serde(rename = "ChecksumSHA256")]
    pub checksum_sha256: Option<String>,

    pub e_tag: Option<String>,

    pub last_modified: Option<DateTime<Utc>>,

    pub part_number: u16,

    pub size: Option<u64>,
}

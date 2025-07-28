use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::types::ChecksumAlgorithm;
use crate::types::ChecksumType;
use crate::types::CommonPrefix;
use crate::types::Initiator;
use crate::types::Owner;
use crate::types::Part;
use crate::types::StorageClass;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListPartsResult {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    pub bucket: Option<String>,

    pub checksum_algorithm: Option<ChecksumAlgorithm>,

    pub checksum_type: Option<ChecksumType>,

    pub common_prefixes: Vec<CommonPrefix>,

    pub initiator: Option<Initiator>,

    pub is_truncated: bool,

    pub key: String,

    pub max_parts: u16,

    pub next_part_number_marker: Option<String>,

    pub owner: Option<Owner>,

    pub part: Vec<Part>,

    pub part_number_marker: Option<u16>,

    pub storage_class: Option<StorageClass>,

    pub upload_id: Uuid,
}

use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::types::ChecksumAlgorithm;
use crate::types::ChecksumType;
use crate::types::Initiator;
use crate::types::Owner;
use crate::types::StorageClass;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Builder, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MultipartUpload {
    pub upload_id: Option<Uuid>,

    pub key: Option<String>,

    pub initiated: Option<DateTime<Utc>>,

    pub storage_class: Option<StorageClass>,

    pub owner: Option<Owner>,

    pub initiator: Option<Initiator>,

    pub checksum_algorithm: Option<ChecksumAlgorithm>,

    pub checksum_type: Option<ChecksumType>,
}

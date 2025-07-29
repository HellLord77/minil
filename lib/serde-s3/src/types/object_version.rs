use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::types::ChecksumAlgorithm;
use crate::types::ChecksumType;
use crate::types::ObjectVersionStorageClass;
use crate::types::Owner;
use crate::types::RestoreStatus;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectVersion {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,

    pub checksum_type: Option<ChecksumType>,

    pub e_tag: Option<String>,

    pub is_latest: Option<bool>,

    pub key: Option<String>,

    pub last_modified: Option<DateTime<Utc>>,

    pub owner: Option<Owner>,

    pub restore_status: Option<RestoreStatus>,

    pub size: Option<u64>,

    pub storage_class: Option<ObjectVersionStorageClass>,

    pub version_id: Option<Uuid>,
}

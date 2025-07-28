use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::types::Owner;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteMarkerEntry {
    pub is_latest: Option<bool>,

    pub key: Option<String>,

    pub last_modified: Option<DateTime<Utc>>,

    pub owner: Option<Owner>,

    pub version_id: Option<String>,
}

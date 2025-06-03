use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RestoreStatus {
    pub is_restore_in_progress: Option<bool>,

    pub restore_expiry_date: Option<DateTime<Utc>>,
}

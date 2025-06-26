use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bucket {
    pub bucket_region: Option<String>,

    pub creation_date: Option<DateTime<Utc>>,

    pub name: Option<String>,
}

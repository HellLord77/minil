use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use validator::Validate;

#[serde_inline_default]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListBucketsQuery {
    pub bucket_region: Option<String>,
    pub continuation_token: Option<String>,
    #[validate(range(min = 1, max = 10_000))]
    #[serde_inline_default(10_000)]
    pub max_buckets: u16,
    #[validate(length(min = 0, max = 1_024))]
    pub prefix: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bucket {
    pub bucket_region: String,
    pub creation_date: DateTime<Utc>,
    pub name: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Owner {
    pub display_name: String,
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAllMyBucketsResult {
    pub buckets: Vec<Bucket>,
    pub owner: Owner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

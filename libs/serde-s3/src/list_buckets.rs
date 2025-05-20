use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_inline_default::serde_inline_default;
use serde_with::skip_serializing_none;
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

#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAllMyBucketsResult {
    pub buckets: Vec<Bucket>,

    pub owner: Owner,

    pub continuation_token: Option<String>,

    pub prefix: Option<String>,
}

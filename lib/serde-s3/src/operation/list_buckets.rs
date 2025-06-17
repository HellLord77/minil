use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use validator::Validate;

use crate::types::ListAllMyBucketsResult;

#[serde_inline_default]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListBucketsInputQuery {
    pub bucket_region: Option<String>,

    pub continuation_token: Option<String>,

    #[validate(range(min = 1, max = 10_000))]
    #[serde_inline_default(10_000)]
    pub max_buckets: u16,

    #[validate(length(min = 0, max = 1_024))]
    #[serde(default)]
    pub prefix: String,
}

pub type ListBucketsOutputBody = ListAllMyBucketsResult;

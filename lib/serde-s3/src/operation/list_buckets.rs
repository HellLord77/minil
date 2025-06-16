use derive_getters::Getters;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use validator::Validate;

use crate::types::ListAllMyBucketsResult;

#[serde_inline_default]
#[derive(Debug, Getters, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListBucketsInputQuery {
    bucket_region: Option<String>,

    continuation_token: Option<String>,

    #[validate(range(min = 1, max = 10_000))]
    #[serde_inline_default(10_000)]
    max_buckets: u16,

    #[validate(length(min = 0, max = 1_024))]
    #[serde(default)]
    prefix: String,
}

pub type ListBucketsOutputBody = ListAllMyBucketsResult;

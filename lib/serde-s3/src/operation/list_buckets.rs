use bon::Builder;
use serde::Serialize;
use serde_inline_default::serde_inline_default;
use serde_with::skip_serializing_none;
use serdev::Deserialize;
use validator::Validate;

use crate::types::Owner;
use crate::utils::Buckets;

#[serde_inline_default]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct ListBucketsInputQuery {
    pub bucket_region: Option<String>,

    pub continuation_token: Option<String>,

    #[validate(range(min = 1, max = 10_000))]
    #[serde_inline_default(10_000)]
    pub max_buckets: u16,

    #[validate(length(min = 0, max = 1_024))]
    pub prefix: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "ListAllMyBucketsResult", rename_all = "PascalCase")]
pub struct ListBucketsOutputBody {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    #[builder(into)]
    pub buckets: Option<Buckets>,

    pub owner: Option<Owner>,

    pub continuation_token: Option<String>,

    pub prefix: Option<String>,
}

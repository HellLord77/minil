use bon::Builder;
use serde::Serialize;
use serde_inline_default::serde_inline_default;
use serde_rename_chain::serde_rename_chain;
use serde_with::skip_serializing_none;
use serdev::Deserialize;
use validator::Validate;
use validator_extra::validate_extra;

use crate::types::CommonPrefix;
use crate::types::EncodingType;
use crate::types::Object;
use crate::types::OptionalObjectAttributes;
use crate::types::RequestCharged;
use crate::types::RequestPayer;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListObjectsV2InputPath {
    pub bucket: String,
}

#[validate_extra]
#[serde_inline_default]
#[serde_rename_chain(convert_case = "kebab")]
#[derive(Debug, Validate, Deserialize)]
#[serde(validate = "Validate::validate")]
pub struct ListObjectsV2InputQuery {
    pub continuation_token: Option<String>,

    #[validate_extra(eq(other = "/"))]
    pub delimiter: Option<String>,

    pub encoding_type: Option<EncodingType>,

    pub fetch_owner: Option<bool>,

    #[validate(range(min = 1, max = 1_000))]
    #[serde_inline_default(1_000)]
    pub max_keys: u16,

    #[validate(length(min = 0, max = 1_024))]
    pub prefix: Option<String>,

    pub start_after: Option<String>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct ListObjectsV2InputHeader {
    pub expected_bucket_owner: Option<String>,

    pub optional_object_attributes: Option<Vec<OptionalObjectAttributes>>,

    pub request_payer: Option<RequestPayer>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct ListObjectsV2OutputHeader {
    pub request_charged: Option<RequestCharged>,
}

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "ListBucketResult", rename_all = "PascalCase")]
pub struct ListObjectsV2OutputBody {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    pub common_prefixes: Vec<CommonPrefix>,

    pub contents: Vec<Object>,

    pub continuation_token: Option<String>,

    pub delimiter: Option<String>,

    pub encoding_type: Option<EncodingType>,

    pub is_truncated: bool,

    pub key_count: u16,

    pub max_keys: u16,

    pub name: String,

    pub next_continuation_token: Option<String>,

    pub prefix: String,

    pub start_after: Option<String>,
}

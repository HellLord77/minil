use derive_getters::Getters;
use serde::Deserialize;
use serde::Serialize;
use serde_inline_default::serde_inline_default;
use serde_rename_chain::serde_rename_chain;
use validator::Validate;

use crate::types::EncodingType;
use crate::types::ListBucketResult;
use crate::types::OptionalObjectAttributes;
use crate::types::RequestPayer;

#[serde_inline_default]
#[derive(Debug, Getters, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListObjectsInputQuery {
    delimiter: Option<String>,

    encoding_type: Option<EncodingType>,

    marker: Option<String>,

    #[validate(range(min = 1, max = 1_000))]
    #[serde_inline_default(1_000)]
    max_keys: u16,

    #[validate(length(min = 0, max = 1_024))]
    #[serde(default)]
    prefix: String,
}

#[serde_rename_chain(add_prefix = "x_amz_", ident_case = "kebab")]
#[derive(Debug, Getters, Deserialize)]
pub struct ListObjectsInputHeader {
    expected_bucket_owner: Option<String>,

    request_payer: Option<RequestPayer>,

    optional_object_attributes: Vec<OptionalObjectAttributes>,
}

#[serde_rename_chain(add_prefix = "x_amz_", ident_case = "kebab")]
#[derive(Debug, Serialize)]
pub struct ListObjectsOutputHeader {
    pub request_charged: Option<RequestPayer>,
}

pub type ListObjectsOutputBody = ListBucketResult;

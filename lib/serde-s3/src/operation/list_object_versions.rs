use bon::Builder;
use serde::Serialize;
use serde_inline_default::serde_inline_default;
use serde_rename_chain::serde_rename_chain;
use serdev::Deserialize;
use validator::Validate;
use validator_extra::validate_extra;

use crate::types::EncodingType;
use crate::types::ListVersionsResult;
use crate::types::OptionalObjectAttributes;
use crate::types::RequestPayer;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListObjectVersionsInputPath {
    pub bucket: String,
}

#[validate_extra]
#[serde_inline_default]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct ListObjectVersionsInputQuery {
    #[validate_extra(eq(other = "/"))]
    pub delimiter: Option<String>,

    pub encoding_type: Option<EncodingType>,

    pub key_marker: Option<String>,

    #[validate(range(min = 1, max = 1_000))]
    #[serde_inline_default(1_000)]
    pub max_keys: u16,

    #[validate(length(min = 0, max = 1_024))]
    pub prefix: Option<String>,

    pub version_id_marker: Option<String>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct ListObjectVersionsInputHeader {
    pub expected_bucket_owner: Option<String>,

    pub optional_object_attributes: Option<Vec<OptionalObjectAttributes>>,

    pub request_payer: Option<RequestPayer>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct ListObjectVersionsOutputHeader {
    pub request_charged: Option<RequestPayer>,
}

pub type ListObjectVersionsOutputBody = ListVersionsResult;

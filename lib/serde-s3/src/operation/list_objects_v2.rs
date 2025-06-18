use serde::Serialize;
use serde_inline_default::serde_inline_default;
use serde_rename_chain::serde_rename_chain;
use serdev::Deserialize;
use validator::Validate;

use crate::types::ListBucketResultV2;
use crate::types::OptionalObjectAttributes;
use crate::types::RequestPayer;

#[serde_inline_default]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct ListObjectsV2InputQuery {
    #[validate(range(min = 2, max = 2))]
    pub list_type: u8,

    pub continuation_token: Option<String>,

    pub delimiter: Option<String>,

    pub encoding_type: Option<String>,

    #[serde_inline_default(false)]
    pub fetch_owner: bool,

    #[validate(range(min = 1, max = 1_000))]
    #[serde_inline_default(1_000)]
    pub max_keys: u16,

    #[validate(length(min = 0, max = 1_024))]
    #[serde(default)]
    pub prefix: String,

    pub start_after: Option<String>,
}

#[serde_rename_chain(add_prefix = "x_amz_", ident_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct ListObjectsV2InputHeader {
    pub expected_bucket_owner: Option<String>,

    #[serde(default)]
    pub optional_object_attributes: Vec<OptionalObjectAttributes>,

    pub request_payer: Option<RequestPayer>,
}

#[serde_rename_chain(add_prefix = "x_amz_", ident_case = "kebab")]
#[derive(Debug, Serialize)]
pub struct ListObjectsV2OutputHeader {
    pub request_charged: Option<RequestPayer>,
}

pub type ListObjectsV2OutputBody = ListBucketResultV2;

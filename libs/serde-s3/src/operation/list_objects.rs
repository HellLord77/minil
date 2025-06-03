use crate::types::EncodingType;
use crate::types::ListBucketResult;
use crate::types::OptionalObjectAttributes;
use crate::types::RequestPayer;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_rename_chain::serde_rename_chain;
use validator::Validate;

#[serde_inline_default]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListObjectsInputQuery {
    pub delimiter: Option<String>,

    pub encoding_type: Option<EncodingType>,

    pub marker: Option<String>,

    #[validate(range(min = 1, max = 1_000))]
    #[serde_inline_default(1_000)]
    pub max_keys: u16,

    #[validate(length(min = 0, max = 1_024))]
    #[serde(default)]
    pub prefix: String,
}

#[serde_rename_chain(add_prefix = "x_amz_", ident_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct ListObjectsInputHeader {
    pub expected_bucket_owner: Option<String>,

    pub request_payer: Option<RequestPayer>,

    pub optional_object_attributes: Vec<OptionalObjectAttributes>,
}

pub type ListObjectsOutputBody = ListBucketResult;

use bon::Builder;
use serde::Serialize;
use serde_inline_default::serde_inline_default;
use serde_rename_chain::serde_rename_chain;
use serde_with::skip_serializing_none;
use serdev::Deserialize;
use uuid::Uuid;
use validator::Validate;
use validator_extra::validate_extra;

use crate::types::CommonPrefix;
use crate::types::DeleteMarkerEntry;
use crate::types::EncodingType;
use crate::types::ObjectVersion;
use crate::types::OptionalObjectAttributes;
use crate::types::RequestCharged;
use crate::types::RequestPayer;
use crate::utils::DeleteMarkerOrVersion;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListObjectVersionsInputPath {
    pub bucket: String,
}

#[validate_extra]
#[serde_inline_default]
#[serde_rename_chain(convert_case = "kebab")]
#[derive(Debug, Validate, Deserialize)]
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

    pub version_id_marker: Option<String>, // fixme
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
    pub request_charged: Option<RequestCharged>,
}

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "ListVersionsResult", rename_all = "PascalCase")]
pub struct ListObjectVersionsOutputBody {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    pub common_prefixes: Vec<CommonPrefix>,

    pub delete_marker: Vec<DeleteMarkerEntry>,

    pub delimiter: Option<String>,

    pub encoding_type: Option<EncodingType>,

    pub is_truncated: bool,

    pub key_marker: String,

    pub max_keys: u16,

    pub name: String,

    pub next_key_marker: Option<String>,

    pub next_version_id_marker: Option<Uuid>,

    pub prefix: String,

    pub version: Vec<ObjectVersion>,

    pub version_id_marker: String, // fixme

    #[serde(rename = "$value")]
    pub delete_marker_or_version: Vec<DeleteMarkerOrVersion>,
}

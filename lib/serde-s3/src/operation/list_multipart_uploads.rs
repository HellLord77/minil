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
use crate::types::MultipartUpload;
use crate::types::RequestCharged;
use crate::types::RequestPayer;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListMultipartUploadsInputPath {
    pub bucket: String,
}

#[validate_extra]
#[serde_inline_default]
#[serde_rename_chain(convert_case = "kebab")]
#[derive(Debug, Validate, Deserialize)]
#[serde(validate = "Validate::validate")]
pub struct ListMultipartUploadsInputQuery {
    #[validate_extra(eq(other = "/"))]
    pub delimiter: Option<String>,

    pub encoding_type: Option<EncodingType>,

    pub key_marker: Option<String>,

    #[validate(range(min = 1, max = 1_000))]
    #[serde_inline_default(1_000)]
    pub max_uploads: u16,

    #[validate(length(min = 0, max = 1_024))]
    pub prefix: Option<String>,

    pub upload_id_marker: Option<String>, // fixme
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct ListMultipartUploadsInputHeader {
    pub expected_bucket_owner: Option<String>,

    pub request_payer: Option<RequestPayer>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct ListMultipartUploadsOutputHeader {
    pub request_charged: Option<RequestCharged>,
}

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "ListMultipartUploadsResult", rename_all = "PascalCase")]
pub struct ListMultipartUploadsOutputBody {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    pub bucket: String,

    pub common_prefixes: Vec<CommonPrefix>,

    pub delimiter: Option<String>,

    pub encoding_type: Option<EncodingType>,

    pub is_truncated: bool,

    pub key_marker: String,

    pub max_uploads: u16,

    pub next_key_marker: String,

    pub next_upload_id_marker: String, // fixme

    pub prefix: Option<String>,

    pub upload: Vec<MultipartUpload>,

    pub upload_id_marker: String, // fixme
}

use bon::Builder;
use http_digest::DigestMd5;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;
use serde_with::serde_as;
use serde_with_extra::DisplayFromBytes;
use serdev::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::types::ChecksumAlgorithm;
use crate::types::RequestPayer;
use crate::types::Tagging;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct PutObjectTaggingInputPath {
    pub bucket: String,

    #[validate(length(min = 1))]
    pub key: String,
}

#[serde_rename_chain]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PutObjectTaggingInputQuery {
    #[serde_rename_chain(convert_case = "camel")]
    pub version_id: Option<Uuid>,
}

#[serde_as]
#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct PutObjectTaggingInputHeader {
    #[serde(rename = "Content-MD5")]
    #[serde_as(as = "Option<DisplayFromBytes>")]
    pub content_md5: Option<DigestMd5>,

    pub expected_bucket_owner: Option<String>,

    pub request_payer: Option<RequestPayer>,

    pub sdk_checksum_algorithm: Option<ChecksumAlgorithm>,
}

pub type PutObjectTaggingInputBody = Tagging;

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct PutObjectTaggingOutputHeader {
    pub version_id: Option<Uuid>,
}

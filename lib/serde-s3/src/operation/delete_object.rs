use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;
use serdev::Deserialize;
use validator::Validate;

use crate::types::RequestCharged;
use crate::types::RequestPayer;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct DeleteObjectInputPath {
    pub bucket: String,

    #[validate(length(min = 1))]
    pub key: String,
}

#[serde_rename_chain]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeleteObjectInputQuery {
    #[serde_rename_chain(convert_case = "camel")]
    pub version_id: Option<String>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct DeleteObjectInputHeader {
    #[serde_rename_chain(convert_case = "train")]
    pub if_match: Option<String>,

    pub bypass_governance_retention: Option<bool>,

    pub expected_bucket_owner: Option<String>,

    pub if_match_last_modified_time: Option<DateTime<Utc>>,

    pub if_match_size: Option<u64>,

    pub mfa: Option<String>,

    pub request_payer: Option<RequestPayer>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct DeleteObjectOutputHeader {
    pub delete_marker: Option<bool>,

    pub request_charged: Option<RequestCharged>,

    pub version_id: Option<String>,
}

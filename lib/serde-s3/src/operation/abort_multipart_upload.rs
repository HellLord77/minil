use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;
use serdev::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::types::RequestCharged;
use crate::types::RequestPayer;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct AbortMultipartUploadInputPath {
    pub bucket: String,

    #[validate(length(min = 1))]
    pub key: String,
}

#[serde_rename_chain(convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct AbortMultipartUploadInputQuery {
    #[serde_rename_chain(convert_case = "camel")]
    pub upload_id: Uuid,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct AbortMultipartUploadInputHeader {
    pub expected_bucket_owner: Option<String>,

    pub if_match_initiated_time: Option<DateTime<Utc>>,

    pub request_payer: Option<RequestPayer>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct AbortMultipartUploadOutputHeader {
    pub request_charged: Option<RequestCharged>,
}

use bon::Builder;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;
use serdev::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::types::RequestPayer;
use crate::types::Tagging;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct GetObjectTaggingInputPath {
    pub bucket: String,

    #[validate(length(min = 1))]
    pub key: String,
}

#[serde_rename_chain(convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct GetObjectTaggingInputQuery {
    #[serde_rename_chain(convert_case = "camel")]
    pub version_id: Option<Uuid>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct GetObjectTaggingInputHeader {
    pub expected_bucket_owner: Option<String>,

    pub request_payer: Option<RequestPayer>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct GetObjectTaggingOutputHeader {
    pub version_id: Option<Uuid>,
}

pub type GetObjectTaggingOutputBody = Tagging;

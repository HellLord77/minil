use serde::Serialize;
use serde_rename_chain::serde_rename_chain;
use serdev::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct DeleteObjectTaggingInputPath {
    pub bucket: String,

    #[validate(length(min = 1))]
    pub key: String,
}

#[serde_rename_chain]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeleteObjectTaggingInputQuery {
    #[serde_rename_chain(convert_case = "camel")]
    pub version_id: Option<Uuid>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct DeleteObjectTaggingInputHeader {
    pub expected_bucket_owner: Option<String>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Serialize)]
pub struct DeleteObjectTaggingOutputHeader {
    pub version_id: Option<Uuid>,
}

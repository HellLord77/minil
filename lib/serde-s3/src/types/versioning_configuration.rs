use bon::Builder;
use serde_with::skip_serializing_none;
use serdev::Deserialize;
use serdev::Serialize;
use validator::Validate;
use validator_extra::validate_extra;

use crate::types::BucketVersioningStatus;
use crate::types::MfaDeleteStatus;

#[validate_extra]
#[skip_serializing_none]
#[derive(Debug, Builder, Validate, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct VersioningConfiguration {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/".to_owned())]
    #[validate_extra(eq(other = "http://s3.amazonaws.com/doc/2006-03-01/"))]
    #[serde(rename = "@xmlns")]
    pub xmlns: String, // todo &'static str

    pub mfa_delete: Option<MfaDeleteStatus>,

    pub status: Option<BucketVersioningStatus>,
}

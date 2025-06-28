use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::types::BucketVersioningStatus;
use crate::types::MfaDeleteStatus;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VersioningConfiguration {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    #[serde(rename = "MFADelete")]
    mfa_delete: Option<MfaDeleteStatus>,

    status: Option<BucketVersioningStatus>,
}

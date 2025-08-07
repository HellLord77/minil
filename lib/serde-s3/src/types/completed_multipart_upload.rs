use serde_with::skip_serializing_none;
use serdev::Deserialize;
use validator::Validate;
use validator_extra::validate_extra;

use crate::types::CompletedPart;

#[validate_extra]
#[skip_serializing_none]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename = "CompleteMultipartUpload", rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct CompletedMultipartUpload {
    #[validate_extra(eq(other = "http://s3.amazonaws.com/doc/2006-03-01/"))]
    #[serde(rename = "@xmlns")]
    pub xmlns: Option<String>,

    pub parts: Vec<CompletedPart>,
}

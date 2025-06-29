use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::types::BucketLocationConstraint;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationConstraintResult {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    #[serde(rename = "$text")]
    pub content: BucketLocationConstraint,
}

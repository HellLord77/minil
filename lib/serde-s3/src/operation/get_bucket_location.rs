use bon::Builder;
use serde::Deserialize;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;
use serde_with::skip_serializing_none;

use crate::types::BucketLocationConstraint;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBucketLocationInputPath {
    pub bucket: String,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct GetBucketLocationInputHeader {
    pub expected_bucket_owner: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "LocationConstraintResult", rename_all = "PascalCase")]
pub struct GetBucketLocationOutputBody {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    #[serde(rename = "$text")]
    pub content: BucketLocationConstraint,
}

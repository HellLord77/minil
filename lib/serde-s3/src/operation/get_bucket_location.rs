use serde::Deserialize;
use serde_rename_chain::serde_rename_chain;

use crate::types::LocationConstraintResult;

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

pub type GetBucketLocationOutputBody = LocationConstraintResult;

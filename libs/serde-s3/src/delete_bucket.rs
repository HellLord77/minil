use serde::Deserialize;
use serde_rename_chain::serde_rename_chain;

pub const EXPECTED_BUCKET_OWNER: &str = "x-amz-expected-bucket-owner";

#[serde_rename_chain(add_prefix = "x_amz_", rename_rule = "kebab")]
#[derive(Debug, Deserialize)]
pub struct DeleteBucketHeader {
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteBucketQuery {
    pub bucket: String,
}

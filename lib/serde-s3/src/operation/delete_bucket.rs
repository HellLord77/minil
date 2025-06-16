use derive_getters::Getters;
use serde::Deserialize;
use serde_rename_chain::serde_rename_chain;

#[derive(Debug, Getters, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteBucketInputQuery {
    bucket: String,
}

#[serde_rename_chain(add_prefix = "x_amz_", ident_case = "kebab")]
#[derive(Debug, Getters, Deserialize)]
pub struct DeleteBucketInputHeader {
    expected_bucket_owner: Option<String>,
}

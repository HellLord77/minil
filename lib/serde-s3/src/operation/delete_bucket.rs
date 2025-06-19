use serde::Deserialize;
use serde_rename_chain::serde_rename_chain;

#[serde_rename_chain(add_prefix = "x_amz_", ident_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct DeleteBucketInputHeader {
    pub expected_bucket_owner: Option<String>,
}

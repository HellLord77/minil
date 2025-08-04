use serde::Deserialize;
use serde_rename_chain::serde_rename_chain;

use crate::types::ChecksumAlgorithm;
use crate::types::Tagging;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PutBucketTaggingInputPath {
    pub bucket: String,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct PutBucketTaggingInputHeader {
    #[serde(rename = "Content-MD5")]
    pub content_md5: Option<String>,

    pub expected_bucket_owner: Option<String>,

    pub sdk_checksum_algorithm: Option<ChecksumAlgorithm>,
}

pub type PutBucketTaggingInputBody = Tagging;

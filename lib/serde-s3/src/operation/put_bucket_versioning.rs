use http_digest::DigestMd5;
use serde::Deserialize;
use serde_rename_chain::serde_rename_chain;
use serde_with::serde_as;
use serde_with_extra::DisplayFromBytes;

use crate::types::ChecksumAlgorithm;
use crate::types::VersioningConfiguration;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PutBucketVersioningInputPath {
    pub bucket: String,
}

#[serde_as]
#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct PutBucketVersioningInputHeader {
    #[serde(rename = "Content-MD5")]
    #[serde_as(as = "Option<DisplayFromBytes>")]
    pub content_md5: Option<DigestMd5>,

    pub expected_bucket_owner: Option<String>,

    pub mfa: Option<String>,

    pub sdk_checksum_algorithm: Option<ChecksumAlgorithm>,
}

pub type PutBucketVersioningInputBody = VersioningConfiguration;

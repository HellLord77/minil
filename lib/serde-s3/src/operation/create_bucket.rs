use bon::Builder;
use serde::Deserialize;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;

use crate::types::BucketCannedAcl;
use crate::types::CreateBucketConfiguration;
use crate::types::ObjectOwnership;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateBucketInputPath {
    pub bucket: String,
}

#[serde_rename_chain(add_prefix = "x_amz_", ident_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct CreateBucketInputHeader {
    pub acl: Option<BucketCannedAcl>,

    pub bucket_object_lock_enabled: Option<bool>,

    pub grant_full_control: Option<String>,

    pub grant_read: Option<String>,

    pub grant_read_acp: Option<String>,

    pub grant_write: Option<String>,

    pub grant_write_acp: Option<String>,

    pub object_ownership: Option<ObjectOwnership>,
}

pub type CreateBucketInputBody = CreateBucketConfiguration;

#[serde_rename_chain(add_prefix = "x_amz_", ident_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct CreateBucketOutputHeader {
    #[serde_rename_chain(convert_case = "train")]
    pub location: String,
}

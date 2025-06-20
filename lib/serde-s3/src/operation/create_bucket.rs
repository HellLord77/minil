use serde::Deserialize;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;

use crate::types::BucketCannedAcl;
use crate::types::CreateBucketConfiguration;
use crate::types::ObjectOwnership;

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
#[derive(Debug, Serialize)]
pub struct CreateBucketOutputHeader {
    pub location: String,
}

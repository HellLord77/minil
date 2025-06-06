use derive_getters::Getters;
use serde::Deserialize;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;

use crate::types::BucketCannedAcl;
use crate::types::CreateBucketConfiguration;
use crate::types::ObjectOwnership;

#[serde_rename_chain(add_prefix = "x_amz_", ident_case = "kebab")]
#[derive(Debug, Getters, Deserialize)]
pub struct CreateBucketInputHeader {
    acl: Option<BucketCannedAcl>,

    bucket_object_lock_enabled: Option<bool>,

    grant_full_control: Option<String>,

    grant_read: Option<String>,

    grant_read_acp: Option<String>,

    grant_write: Option<String>,

    grant_write_acp: Option<String>,

    object_ownership: Option<ObjectOwnership>,
}

pub type CreateBucketInputBody = CreateBucketConfiguration;

#[serde_rename_chain(add_prefix = "x_amz_", ident_case = "kebab")]
#[derive(Debug, Serialize)]
pub struct CreateBucketOutputHeader {
    pub location: String,
}

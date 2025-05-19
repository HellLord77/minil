use serde::Deserialize;
use serde_rename_chain::serde_rename_chain;

pub const ACL: &str = "x-amz-acl";
pub const BUCKET_OBJECT_LOCK_ENABLED: &str = "x-amz-bucket-object-lock-enabled";
pub const GRANT_FULL_CONTROL: &str = "x-amz-grant-full-control";
pub const GRANT_READ: &str = "x-amz-grant-read";
pub const GRANT_READ_ACP: &str = "x-amz-grant-read-acp";
pub const GRANT_WRITE: &str = "x-amz-grant-write";
pub const GRANT_WRITE_ACP: &str = "x-amz-grant-write-acp";
pub const OBJECT_OWNERSHIP: &str = "x-amz-object-ownership";

#[derive(Debug, Deserialize)]
pub enum BucketCannedAcl {
    Private,
    PublicRead,
    PublicReadWrite,
    AuthenticatedRead,
}

#[derive(Debug, Deserialize)]
pub enum ObjectOwnership {
    BucketOwnerPreferred,
    ObjectWriter,
    BucketOwnerEnforced,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct CreateBucketHeader {
    pub acl: Option<BucketCannedAcl>,

    pub bucket_object_lock_enabled: Option<bool>,

    pub grant_full_control: Option<String>,

    pub grant_read: Option<String>,

    pub grant_read_acp: Option<String>,

    pub grant_write: Option<String>,

    pub grant_write_acp: Option<String>,

    pub object_ownership: Option<ObjectOwnership>,
}

#[derive(Debug, Deserialize)]
pub enum BucketDataRedundancy {
    SingleAvailabilityZone,
    SingleLocalZone,
}

#[derive(Debug, Deserialize)]
pub enum BucketType {
    Directory,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketInfo {
    pub data_redundancy: Option<BucketDataRedundancy>,

    pub r#type: Option<BucketType>,
}

#[derive(Debug, Deserialize)]
pub enum LocationType {
    AvailabilityZone,
    LocalZone,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationInfo {
    pub name: Option<String>,

    pub r#type: Option<LocationType>,
}

#[derive(Debug, Deserialize)]
pub enum BucketLocationConstraint {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateBucketConfiguration {
    pub bucket: Option<BucketInfo>,

    pub location: Option<LocationInfo>,

    pub location_constraint: Option<BucketLocationConstraint>,
}

use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CreateBucketHeader {
    pub x_amz_acl: Option<BucketCannedAcl>,
    pub x_amz_bucket_object_lock_enabled: Option<bool>,
    pub x_amz_grant_full_control: Option<String>,
    pub x_amz_grant_read: Option<String>,
    pub x_amz_grant_read_acp: Option<String>,
    pub x_amz_grant_write: Option<String>,
    pub x_amz_grant_write_acp: Option<String>,
    pub x_amz_object_ownership: Option<ObjectOwnership>,
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
    pub r#type_: Option<LocationType>,
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

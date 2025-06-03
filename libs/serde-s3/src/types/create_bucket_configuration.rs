use crate::types::BucketInfo;
use crate::types::BucketLocationConstraint;
use crate::types::LocationInfo;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateBucketConfiguration {
    pub bucket: Option<BucketInfo>,

    pub location: Option<LocationInfo>,

    pub location_constraint: Option<BucketLocationConstraint>,
}

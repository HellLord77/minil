use derive_getters::Getters;
use serde::Deserialize;

use crate::types::BucketInfo;
use crate::types::BucketLocationConstraint;
use crate::types::LocationInfo;

#[derive(Debug, Getters, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateBucketConfiguration {
    bucket: Option<BucketInfo>,

    location: Option<LocationInfo>,

    location_constraint: Option<BucketLocationConstraint>,
}

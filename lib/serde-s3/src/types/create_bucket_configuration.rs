use serdev::Deserialize;
use validator::Validate;
use validator_extra::validate_extra;

use crate::types::BucketInfo;
use crate::types::BucketLocationConstraint;
use crate::types::LocationInfo;

#[validate_extra]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct CreateBucketConfiguration {
    #[validate_extra(eq(other = "http://s3.amazonaws.com/doc/2006-03-01/"))]
    #[serde(rename = "@xmlns")]
    pub xmlns: String, // todo &'static str

    pub bucket: Option<BucketInfo>,

    pub location: Option<LocationInfo>,

    pub location_constraint: Option<BucketLocationConstraint>,
}

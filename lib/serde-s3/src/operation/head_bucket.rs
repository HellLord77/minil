use bon::Builder;
use serde::Deserialize;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;

use crate::types::LocationType;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HeadBucketInputPath {
    pub bucket: String,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct HeadBucketInputHeader {
    pub expected_bucket_owner: Option<String>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct HeadBucketOutputHeader {
    pub access_point_alias: Option<bool>,

    pub bucket_location_name: Option<String>,

    pub bucket_location_type: Option<LocationType>,

    pub bucket_region: Option<String>,
}

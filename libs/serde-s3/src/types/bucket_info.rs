use derive_getters::Getters;
use serde::Deserialize;

use crate::types::BucketDataRedundancy;
use crate::types::BucketType;

#[derive(Debug, Getters, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketInfo {
    data_redundancy: Option<BucketDataRedundancy>,

    r#type: Option<BucketType>,
}

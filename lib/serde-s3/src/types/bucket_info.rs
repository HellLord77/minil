use serde::Deserialize;

use crate::types::BucketDataRedundancy;
use crate::types::BucketType;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketInfo {
    pub data_redundancy: Option<BucketDataRedundancy>,

    pub r#type: Option<BucketType>,
}

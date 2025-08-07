use serde::Deserialize;
use serde_with::skip_serializing_none;

use crate::types::BucketType;
use crate::types::DataRedundancy;

#[skip_serializing_none]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketInfo {
    pub data_redundancy: Option<DataRedundancy>,

    pub r#type: Option<BucketType>,
}

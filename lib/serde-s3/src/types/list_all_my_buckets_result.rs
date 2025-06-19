use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::types::Bucket;
use crate::types::Owner;

#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAllMyBucketsResult {
    pub buckets: Vec<Bucket>,

    pub owner: Option<Owner>,

    pub continuation_token: Option<String>,

    pub prefix: Option<String>,
}

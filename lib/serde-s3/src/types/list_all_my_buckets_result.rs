use serde::Serialize;
use serde_with::skip_serializing_none;
use smart_default::SmartDefault;

use crate::types::Bucket;
use crate::types::Owner;

#[skip_serializing_none]
#[derive(Debug, SmartDefault, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAllMyBucketsResult {
    #[default = "http://s3.amazonaws.com/doc/2006-03-01/"]
    #[serde(rename = "@xmlns")]
    pub xmlns: String,

    pub buckets: Vec<Bucket>,

    pub owner: Option<Owner>,

    pub continuation_token: Option<String>,

    pub prefix: Option<String>,
}

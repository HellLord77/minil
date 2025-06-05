use serde::Serialize;

use crate::types::CommonPrefix;
use crate::types::EncodingType;
use crate::types::Object;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListBucketResult {
    pub common_prefixes: Vec<CommonPrefix>,

    pub contents: Vec<Object>,

    pub delimiter: Option<String>,

    pub encoding_type: Option<EncodingType>,

    pub is_truncated: bool,

    pub marker: String,

    pub max_keys: u16,

    pub name: String,

    pub next_marker: Option<String>,

    pub prefix: String,
}

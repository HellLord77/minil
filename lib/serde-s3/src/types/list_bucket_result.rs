use serde::Serialize;
use serde_with::skip_serializing_none;
use smart_default::SmartDefault;

use crate::types::CommonPrefix;
use crate::types::EncodingType;
use crate::types::Object;

#[skip_serializing_none]
#[derive(Debug, SmartDefault, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListBucketResult {
    #[default = "http://s3.amazonaws.com/doc/2006-03-01/"]
    #[serde(rename = "@xmlns")]
    pub xmlns: String,

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

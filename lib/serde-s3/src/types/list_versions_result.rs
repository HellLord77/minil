use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::types::CommonPrefix;
use crate::types::DeleteMarkerEntry;
use crate::types::EncodingType;
use crate::types::ObjectVersion;
use crate::utils::DeleteMarkerOrVersion;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListVersionsResult {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    pub common_prefixes: Vec<CommonPrefix>,

    pub delete_marker: Vec<DeleteMarkerEntry>,

    pub delimiter: Option<String>,

    pub encoding_type: Option<EncodingType>,

    pub is_truncated: bool,

    pub key_marker: String,

    pub max_keys: u16,

    pub name: String,

    pub next_key_marker: Option<String>,

    pub next_version_id_marker: Option<Uuid>,

    pub prefix: String,

    pub version: Vec<ObjectVersion>,

    pub version_id_marker: String,

    #[serde(rename = "$value")]
    pub _delete_marker_or_version: Vec<DeleteMarkerOrVersion>,
}

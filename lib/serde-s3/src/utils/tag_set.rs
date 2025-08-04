use serde::Deserialize;
use serde::Serialize;

use crate::types::Tag;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagSet {
    pub tag: Vec<Tag>,
}

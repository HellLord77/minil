use derive_more::From;
use serde::Deserialize;
use serde::Serialize;

use crate::types::Tag;

#[derive(Debug, From, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagSet {
    pub tag: Vec<Tag>,
}

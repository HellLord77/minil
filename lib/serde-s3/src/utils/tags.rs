use serde::Deserialize;

use crate::types::Tag;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tags {
    pub tag: Vec<Tag>,
}

use serde::Deserialize;

use crate::types::LocationType;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationInfo {
    pub name: Option<String>,

    pub r#type: Option<LocationType>,
}

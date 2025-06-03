use crate::types::LocationType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationInfo {
    pub name: Option<String>,

    pub r#type: Option<LocationType>,
}

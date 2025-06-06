use derive_getters::Getters;
use serde::Deserialize;

use crate::types::LocationType;

#[derive(Debug, Getters, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationInfo {
    name: Option<String>,

    r#type: Option<LocationType>,
}

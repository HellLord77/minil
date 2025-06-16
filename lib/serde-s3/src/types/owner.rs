use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Owner {
    pub display_name: Option<String>,

    #[serde(rename = "ID")]
    pub id: Option<String>,
}

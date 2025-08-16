use bon::Builder;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Builder, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Owner {
    pub display_name: Option<String>,

    #[serde(rename = "ID")]
    pub id: Option<Uuid>,
}

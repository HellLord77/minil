use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Initiator {
    pub display_name: Option<String>,

    #[serde(rename = "ID")]
    pub id: Option<Uuid>,
}

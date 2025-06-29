use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use tynm::type_name;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct AccessDenied {
    #[builder(default = type_name::<AccessDenied>())]
    pub code: String,

    #[builder(default = "Access Denied")]
    pub message: &'static str,

    #[builder(into)]
    pub resource: Option<String>,

    #[builder(into)]
    pub request_id: Option<String>,
}

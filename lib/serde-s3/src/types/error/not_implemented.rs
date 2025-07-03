use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use tynm::type_name;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct NotImplemented {
    #[builder(default = type_name::<NotImplemented>())]
    pub code: String,

    #[builder(
        default = "A header that you provided implies functionality that is not implemented."
    )]
    pub message: &'static str,

    #[builder(into)]
    pub resource: Option<String>,

    #[builder(into)]
    pub request_id: Option<String>,
}

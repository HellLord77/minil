use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use stringify_extra::stringify_ty;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct NotImplemented {
    #[builder(default = stringify_ty!(NotImplemented))]
    pub code: &'static str,

    #[builder(
        default = "A header that you provided implies functionality that is not implemented."
    )]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

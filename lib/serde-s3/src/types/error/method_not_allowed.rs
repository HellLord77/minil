use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use stringify_extra::stringify_ty;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct MethodNotAllowed {
    #[builder(default = stringify_ty!(MethodNotAllowed))]
    pub code: &'static str,

    #[builder(default = "The specified method is not allowed against this resource.")]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

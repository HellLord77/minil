use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use stringify_checked::stringify_ty;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct InvalidPart {
    #[builder(default = stringify_ty!(InvalidPart))]
    pub code: &'static str,

    #[builder(default = "One or more of the specified parts could not be found.")]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

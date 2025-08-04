use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use stringify_checked::stringify_ty;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct MalformedXML {
    #[builder(default = stringify_ty!(MalformedXML))]
    pub code: &'static str,

    #[builder(
        default = "The XML that you provided was not well formed or did not validate against our published schema."
    )]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

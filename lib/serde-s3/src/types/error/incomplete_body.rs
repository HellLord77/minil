use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use stringify_checked::stringify_ty;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct IncompleteBody {
    #[builder(default = stringify_ty!(IncompleteBody))]
    pub code: &'static str,

    #[builder(
        default = "You did not provide the number of bytes specified by the Content-Length HTTP header."
    )]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

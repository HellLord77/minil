use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use stringify_checked::stringify_ty;

#[deprecated]
#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct InvalidArgument {
    #[builder(default = stringify_ty!(InvalidArgument))]
    pub code: &'static str,

    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

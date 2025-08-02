use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use stringify_checked::stringify_ty;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct NoSuchVersion {
    #[builder(default = stringify_ty!(NoSuchVersion))]
    pub code: &'static str,

    #[builder(
        default = "The version ID specified in the request does not match an existing version."
    )]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

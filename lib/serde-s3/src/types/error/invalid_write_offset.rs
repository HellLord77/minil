use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use stringify_checked::stringify_ty;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct InvalidWriteOffset {
    #[builder(default = stringify_ty!(InvalidWriteOffset))]
    pub code: &'static str,

    #[builder(
        default = "The write offset value that you specified does not match the current object size."
    )]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

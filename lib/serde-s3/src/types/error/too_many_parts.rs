use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use stringify_checked::stringify_ty;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct TooManyParts {
    #[builder(default = stringify_ty!(TooManyParts))]
    pub code: &'static str,

    #[builder(
        default = "You have attempted to add more parts than the maximum of 10000 that are allowed for this object."
    )]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

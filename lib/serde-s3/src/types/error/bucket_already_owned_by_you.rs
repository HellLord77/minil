use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use stringify_checked::stringify_ty;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct BucketAlreadyOwnedByYou {
    #[builder(default = stringify_ty!(BucketAlreadyOwnedByYou))]
    pub code: &'static str,

    #[builder(default = "The bucket that you tried to create already exists, and you own it.")]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use stringify_checked::stringify_ty;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct EncryptionTypeMismatch {
    #[builder(default = stringify_ty!(EncryptionTypeMismatch))]
    pub code: &'static str,

    #[builder(default = "The existing object was created with a different encryption type.")]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use tynm::type_name;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct EncryptionTypeMismatch {
    #[builder(default = type_name::<EncryptionTypeMismatch>())]
    pub code: String,

    #[builder(default = "The existing object was created with a different encryption type.")]
    pub message: &'static str,

    #[builder(into)]
    pub resource: Option<String>,

    #[builder(into)]
    pub request_id: Option<String>,
}

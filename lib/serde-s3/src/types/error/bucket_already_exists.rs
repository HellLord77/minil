use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use tynm::type_name;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct BucketAlreadyExists {
    #[builder(default = type_name::<BucketAlreadyExists>())]
    pub code: String,

    #[builder(default = "The requested bucket name is not available.")]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

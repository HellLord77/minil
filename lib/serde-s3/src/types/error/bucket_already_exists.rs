use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketAlreadyExists {
    #[builder(default = "BucketAlreadyExists")]
    pub code: &'static str,

    #[builder(default = "The requested bucket name is not available.")]
    pub message: &'static str,

    #[builder(into)]
    pub resource: Option<String>,

    #[builder(into)]
    pub request_id: Option<String>,
}

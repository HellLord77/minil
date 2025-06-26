use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct NoSuchBucket {
    #[builder(default = "NoSuchBucket")]
    pub code: &'static str,

    #[builder(default = "The specified bucket does not exist.")]
    pub message: &'static str,

    #[builder(into)]
    pub resource: Option<String>,

    #[builder(into)]
    pub request_id: Option<String>,
}

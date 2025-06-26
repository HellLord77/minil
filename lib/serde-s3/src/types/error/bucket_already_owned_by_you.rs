use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketAlreadyOwnedByYou {
    #[builder(default = "BucketAlreadyOwnedByYou")]
    pub code: &'static str,

    #[builder(default = "The bucket that you tried to create already exists, and you own it.")]
    pub message: &'static str,

    #[builder(into)]
    pub resource: Option<String>,

    #[builder(into)]
    pub request_id: Option<String>,
}

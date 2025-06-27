use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use tynm::type_name;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct InvalidWriteOffset {
    #[builder(default = type_name::<InvalidWriteOffset>())]
    pub code: String,

    #[builder(
        default = "The write offset value that you specified does not match the current object size."
    )]
    pub message: &'static str,

    #[builder(into)]
    pub resource: Option<String>,

    #[builder(into)]
    pub request_id: Option<String>,
}

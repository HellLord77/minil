use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use tynm::type_name;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct TooManyParts {
    #[builder(default = type_name::<TooManyParts>())]
    pub code: String,

    #[builder(
        default = "You have attempted to add more parts than the maximum of 10000 that are allowed for this object."
    )]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

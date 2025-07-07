use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use tynm::type_name;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct PreconditionFailed {
    #[builder(default = type_name::<PreconditionFailed>())]
    pub code: String,

    #[builder(default = "At least one of the preconditions that you specified did not hold.")]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

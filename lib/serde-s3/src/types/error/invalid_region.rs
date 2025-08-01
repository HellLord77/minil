use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use stringify_checked::stringify_ty;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct InvalidRegion {
    #[builder(default = stringify_ty!(InvalidRegion))]
    pub code: &'static str,

    #[builder(
        default = "You've attempted to create a Multi-Region Access Point in a Region that you haven't opted in to."
    )]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use tynm::type_name;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct BadDigest {
    #[builder(default = type_name::<BadDigest>())]
    pub code: String,

    #[builder(
        default = "The Content-MD5 or checksum value that you specified did not match what the server received."
    )]
    pub message: &'static str,

    #[builder(into)]
    pub resource: Option<String>,

    #[builder(into)]
    pub request_id: Option<String>,
}

use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use stringify_extra::stringify_ty;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "Error", rename_all = "PascalCase")]
pub struct InvalidDigest {
    #[builder(default = stringify_ty!(InvalidDigest))]
    pub code: &'static str,

    #[builder(default = "The Content-MD5 or checksum value that you specified is not valid.")]
    pub message: &'static str,

    pub resource: Option<String>,

    pub request_id: Option<String>,
}

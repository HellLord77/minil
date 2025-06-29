use axum::http::StatusCode;
use axum_into_response::IntoResponse;
use axum_s3_macros::ErrorFromRequestParts;
use axum_xml::Xml;
use bon::Builder;
use serde_s3::types::error::BadDigest;

#[derive(Debug, Builder, IntoResponse, ErrorFromRequestParts)]
pub struct BadDigestOutput {
    #[builder(default = StatusCode::BAD_REQUEST)]
    pub status: StatusCode,

    #[into_response(via(Xml))]
    pub body: BadDigest,
}

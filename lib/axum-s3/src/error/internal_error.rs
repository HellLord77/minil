use axum::http::StatusCode;
use axum_into_response::IntoResponse;
use axum_s3_macros::ErrorFromRequestParts;
use axum_xml::Xml;
use bon::Builder;
use serde_s3::types::error::InternalError;

#[derive(Debug, Builder, IntoResponse, ErrorFromRequestParts)]
pub struct InternalErrorOutput {
    #[builder(default = StatusCode::INTERNAL_SERVER_ERROR)]
    pub status: StatusCode,

    #[into_response(via(Xml))]
    pub body: InternalError,
}

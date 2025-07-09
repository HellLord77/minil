use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_s3_macros::ErrorFromParts;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::types::error::InternalError;

#[derive(Debug, Builder, IntoResponse, ErrorFromParts)]
pub struct InternalErrorOutput {
    #[builder(default = StatusCode::INTERNAL_SERVER_ERROR)]
    pub status: StatusCode,

    #[into_response(via(Xml))]
    pub body: InternalError,
}

use axum::http::StatusCode;
use axum_into_response::IntoResponse;
use axum_s3_macros::ErrorFromParts;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::types::error::ConditionalRequestConflict;

#[derive(Debug, Builder, IntoResponse, ErrorFromParts)]
pub struct ConditionalRequestConflictOutput {
    #[builder(default = StatusCode::CONFLICT)]
    pub status: StatusCode,

    #[into_response(via(Xml))]
    pub body: ConditionalRequestConflict,
}

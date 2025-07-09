use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_s3_macros::ErrorFromParts;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::types::error::EncryptionTypeMismatch;

#[derive(Debug, Builder, IntoResponse, ErrorFromParts)]
pub struct EncryptionTypeMismatchOutput {
    #[builder(default = StatusCode::BAD_REQUEST)]
    pub status: StatusCode,

    #[into_response(via(Xml))]
    pub body: EncryptionTypeMismatch,
}

use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_s3_macros::ErrorFromCommon;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::types::error::TooManyParts;

#[derive(Debug, Builder, IntoResponse, ErrorFromCommon)]
pub struct TooManyPartsOutput {
    #[builder(default = StatusCode::BAD_REQUEST)]
    pub status: StatusCode,

    #[into_response(via(Xml))]
    pub body: TooManyParts,
}

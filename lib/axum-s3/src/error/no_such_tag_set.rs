use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_s3_macros::ErrorFromCommon;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::types::error::NoSuchTagSet;

#[derive(Debug, Builder, IntoResponse, ErrorFromCommon)]
pub struct NoSuchTagSetOutput {
    #[builder(default = StatusCode::NOT_FOUND)]
    pub status: StatusCode,

    #[into_response(via(Xml))]
    pub body: NoSuchTagSet,
}

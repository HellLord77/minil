use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_header::Header;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::GetBucketVersioningInputHeader;
use serde_s3::operation::GetBucketVersioningInputPath;
use serde_s3::operation::GetBucketVersioningOutputBody;

#[derive(Debug, FromRequest)]
pub struct GetBucketVersioningInput {
    #[from_request(via(Path))]
    pub path: GetBucketVersioningInputPath,

    #[from_request(via(Header))]
    pub header: GetBucketVersioningInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct GetBucketVersioningOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,

    #[into_response(via(Xml))]
    pub body: GetBucketVersioningOutputBody,
}

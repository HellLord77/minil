use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_header::Header;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::GetBucketTaggingInputHeader;
use serde_s3::operation::GetBucketTaggingInputPath;
use serde_s3::operation::GetBucketTaggingOutputBody;

#[derive(Debug, FromRequest)]
pub struct GetBucketTaggingInput {
    #[from_request(via(Path))]
    pub path: GetBucketTaggingInputPath,

    #[from_request(via(Header))]
    pub header: GetBucketTaggingInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct GetBucketTaggingOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,

    #[into_response(via(Xml))]
    pub body: GetBucketTaggingOutputBody,
}

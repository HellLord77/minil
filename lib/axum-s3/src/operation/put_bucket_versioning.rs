use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_header::Header;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::PutBucketVersioningInputBody;
use serde_s3::operation::PutBucketVersioningInputHeader;
use serde_s3::operation::PutBucketVersioningInputPath;

#[derive(Debug, FromRequest)]
pub struct PutBucketVersioningInput {
    #[from_request(via(Path))]
    pub path: PutBucketVersioningInputPath,

    #[from_request(via(Header))]
    pub header: PutBucketVersioningInputHeader,

    #[from_request(via(Xml))]
    pub body: PutBucketVersioningInputBody,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct PutBucketVersioningOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,
}

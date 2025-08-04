use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_header::Header;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::PutBucketTaggingInputBody;
use serde_s3::operation::PutBucketTaggingInputHeader;
use serde_s3::operation::PutBucketTaggingInputPath;

#[derive(Debug, FromRequest)]
pub struct PutBucketTaggingInput {
    #[from_request(via(Path))]
    pub path: PutBucketTaggingInputPath,

    #[from_request(via(Header))]
    pub header: PutBucketTaggingInputHeader,

    #[from_request(via(Xml))]
    pub body: PutBucketTaggingInputBody,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct PutBucketTaggingOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,
}

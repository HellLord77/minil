use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_header::Header;
use bon::Builder;
use serde_s3::operation::HeadBucketInputHeader;
use serde_s3::operation::HeadBucketInputPath;
use serde_s3::operation::HeadBucketOutputHeader;

#[derive(Debug, FromRequest)]
pub struct HeadBucketInput {
    #[from_request(via(Path))]
    pub path: HeadBucketInputPath,

    #[from_request(via(Header))]
    pub header: HeadBucketInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct HeadBucketOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,

    #[into_response(via(Header))]
    pub header: HeadBucketOutputHeader,
}

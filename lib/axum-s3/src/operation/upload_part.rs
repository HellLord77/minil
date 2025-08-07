use axum::body::Body;
use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_header::Header;
use bon::Builder;
use serde_s3::operation::UploadPartInputHeader;
use serde_s3::operation::UploadPartInputPath;
use serde_s3::operation::UploadPartOutputHeader;

#[derive(Debug, FromRequest)]
pub struct UploadPartInput {
    #[from_request(via(Path))]
    pub path: UploadPartInputPath,

    #[from_request(via(Header))]
    pub header: UploadPartInputHeader,

    pub body: Body,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct UploadPartOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,

    #[into_response(via(Header))]
    pub header: UploadPartOutputHeader,
}

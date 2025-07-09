use axum::body::Body;
use axum::extract::FromRequest;
use axum::extract::Path;
use axum_derive_macros::IntoResponse;
use axum_header::Header;
use bon::Builder;
use serde_s3::operation::PutObjectInputHeader;
use serde_s3::operation::PutObjectInputPath;
use serde_s3::operation::PutObjectOutputHeader;

#[derive(Debug, FromRequest)]
pub struct PutObjectInput {
    #[from_request(via(Path))]
    pub path: PutObjectInputPath,

    #[from_request(via(Header))]
    pub header: PutObjectInputHeader,

    pub body: Body,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct PutObjectOutput {
    #[into_response(via(Header))]
    pub header: PutObjectOutputHeader,
}

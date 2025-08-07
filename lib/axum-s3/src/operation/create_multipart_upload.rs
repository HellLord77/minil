use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_header::Header;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::CreateMultipartUploadInputHeader;
use serde_s3::operation::CreateMultipartUploadInputPath;
use serde_s3::operation::CreateMultipartUploadOutputBody;
use serde_s3::operation::CreateMultipartUploadOutputHeader;

#[derive(Debug, FromRequest)]
pub struct CreateMultipartUploadInput {
    #[from_request(via(Path))]
    pub path: CreateMultipartUploadInputPath,

    #[from_request(via(Header))]
    pub header: CreateMultipartUploadInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct CreateMultipartUploadOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,

    #[into_response(via(Header))]
    pub header: CreateMultipartUploadOutputHeader,

    #[into_response(via(Xml))]
    pub body: CreateMultipartUploadOutputBody,
}

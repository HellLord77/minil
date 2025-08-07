use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_extra::extract::Query;
use axum_header::Header;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::CompleteMultipartUploadInputBody;
use serde_s3::operation::CompleteMultipartUploadInputHeader;
use serde_s3::operation::CompleteMultipartUploadInputPath;
use serde_s3::operation::CompleteMultipartUploadInputQuery;
use serde_s3::operation::CompleteMultipartUploadOutputBody;
use serde_s3::operation::CompleteMultipartUploadOutputHeader;

#[derive(Debug, FromRequest)]
pub struct CompleteMultipartUploadInput {
    #[from_request(via(Path))]
    pub path: CompleteMultipartUploadInputPath,

    #[from_request(via(Query))]
    pub query: CompleteMultipartUploadInputQuery,

    #[from_request(via(Header))]
    pub header: CompleteMultipartUploadInputHeader,

    #[from_request(via(Xml))]
    pub body: CompleteMultipartUploadInputBody,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct CompleteMultipartUploadOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,

    #[into_response(via(Header))]
    pub header: CompleteMultipartUploadOutputHeader,

    #[into_response(via(Xml))]
    pub body: CompleteMultipartUploadOutputBody,
}

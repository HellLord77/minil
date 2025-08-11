use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_extra::extract::Query;
use axum_header::Header;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::ListMultipartUploadsInputHeader;
use serde_s3::operation::ListMultipartUploadsInputPath;
use serde_s3::operation::ListMultipartUploadsInputQuery;
use serde_s3::operation::ListMultipartUploadsOutputBody;
use serde_s3::operation::ListMultipartUploadsOutputHeader;

#[derive(Debug, FromRequest)]
pub struct ListMultipartUploadsInput {
    #[from_request(via(Path))]
    pub path: ListMultipartUploadsInputPath,

    #[from_request(via(Query))]
    pub query: ListMultipartUploadsInputQuery,

    #[from_request(via(Header))]
    pub header: ListMultipartUploadsInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct ListMultipartUploadsOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,

    #[into_response(via(Header))]
    pub header: ListMultipartUploadsOutputHeader,

    #[into_response(via(Xml))]
    pub body: ListMultipartUploadsOutputBody,
}

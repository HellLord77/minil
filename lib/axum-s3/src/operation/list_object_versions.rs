use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_extra::extract::Query;
use axum_header::Header;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::ListObjectVersionsInputHeader;
use serde_s3::operation::ListObjectVersionsInputPath;
use serde_s3::operation::ListObjectVersionsInputQuery;
use serde_s3::operation::ListObjectVersionsOutputBody;
use serde_s3::operation::ListObjectVersionsOutputHeader;

#[derive(Debug, FromRequest)]
pub struct ListObjectVersionsInput {
    #[from_request(via(Path))]
    pub path: ListObjectVersionsInputPath,

    #[from_request(via(Query))]
    pub query: ListObjectVersionsInputQuery,

    #[from_request(via(Header))]
    pub header: ListObjectVersionsInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct ListObjectVersionsOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,

    #[into_response(via(Header))]
    pub header: ListObjectVersionsOutputHeader,

    #[into_response(via(Xml))]
    pub body: ListObjectVersionsOutputBody,
}

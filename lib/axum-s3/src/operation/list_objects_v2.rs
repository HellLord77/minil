use axum::extract::FromRequest;
use axum::extract::Path;
use axum_derive_macros::IntoResponse;
use axum_extra::extract::Query;
use axum_header::Header;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::ListObjectsV2InputHeader;
use serde_s3::operation::ListObjectsV2InputPath;
use serde_s3::operation::ListObjectsV2InputQuery;
use serde_s3::operation::ListObjectsV2OutputBody;
use serde_s3::operation::ListObjectsV2OutputHeader;

#[derive(Debug, FromRequest)]
pub struct ListObjectsV2Input {
    #[from_request(via(Path))]
    pub path: ListObjectsV2InputPath,

    #[from_request(via(Query))]
    pub query: ListObjectsV2InputQuery,

    #[from_request(via(Header))]
    pub header: ListObjectsV2InputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct ListObjectsV2Output {
    #[into_response(via(Header))]
    pub header: ListObjectsV2OutputHeader,

    #[into_response(via(Xml))]
    pub body: ListObjectsV2OutputBody,
}

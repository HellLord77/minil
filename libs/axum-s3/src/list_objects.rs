use axum::extract::FromRequest;
use axum::extract::Path;
use axum_extra::extract::Query;
use axum_header::Header;
use axum_into_response::IntoResponse;
use axum_xml::Xml;
use derive_getters::Getters;
use serde_s3::operation::ListBucketsOutputBody;
use serde_s3::operation::ListObjectsInputHeader;
use serde_s3::operation::ListObjectsInputQuery;
use serde_s3::operation::ListObjectsOutputHeader;

#[derive(Debug, Getters, FromRequest)]
pub struct ListObjectsInput {
    #[from_request(via(Path))]
    bucket: String,

    #[from_request(via(Query))]
    query: ListObjectsInputQuery,

    #[from_request(via(Header))]
    header: ListObjectsInputHeader,
}

#[derive(Debug, IntoResponse)]
pub struct ListObjectsOutput {
    #[into_response(via(Header))]
    pub header: ListObjectsOutputHeader,

    #[into_response(via(Xml))]
    pub body: ListBucketsOutputBody,
}

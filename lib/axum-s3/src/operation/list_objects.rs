use axum::extract::FromRequest;
use axum::extract::Path;
use axum_extra::extract::Query;
use axum_header::Header;
use axum_into_response::IntoResponse;
use axum_xml::Xml;
use bon::Builder;
use serde_s3::operation::ListObjectsInputHeader;
use serde_s3::operation::ListObjectsInputQuery;
use serde_s3::operation::ListObjectsOutputBody;
use serde_s3::operation::ListObjectsOutputHeader;

#[derive(Debug, FromRequest)]
pub struct ListObjectsInput {
    #[from_request(via(Path))]
    pub bucket: String,

    #[from_request(via(Query))]
    pub query: ListObjectsInputQuery,

    #[from_request(via(Header))]
    pub header: ListObjectsInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct ListObjectsOutput {
    #[into_response(via(Header))]
    pub header: ListObjectsOutputHeader,

    #[into_response(via(Xml))]
    pub body: ListObjectsOutputBody,
}

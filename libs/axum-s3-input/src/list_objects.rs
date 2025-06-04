use axum::extract::FromRequest;
use axum::extract::Path;
use axum_extra::extract::Query;
use axum_header::Header;
use serde_s3::operation::ListObjectsInputHeader;
use serde_s3::operation::ListObjectsInputQuery;

#[derive(Debug, FromRequest)]
pub struct ListObjectsInput {
    #[from_request(via(Path))]
    pub bucket: String,

    #[from_request(via(Query))]
    pub query: ListObjectsInputQuery,

    #[from_request(via(Header))]
    pub header: ListObjectsInputHeader,
}

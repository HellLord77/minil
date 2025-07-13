use axum::extract::FromRequest;
use axum::extract::Path;
use axum_derive_macros::IntoResponse;
use axum_extra::extract::Query;
use axum_header::Header;
use bon::Builder;
use serde_s3::operation::HeadObjectInputHeader;
use serde_s3::operation::HeadObjectInputPath;
use serde_s3::operation::HeadObjectInputQuery;
use serde_s3::operation::HeadObjectOutputHeader;

#[derive(Debug, FromRequest)]
pub struct HeadObjectInput {
    #[from_request(via(Path))]
    pub path: HeadObjectInputPath,

    #[from_request(via(Query))]
    pub query: HeadObjectInputQuery,

    #[from_request(via(Header))]
    pub header: HeadObjectInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct HeadObjectOutput {
    #[into_response(via(Header))]
    pub header: HeadObjectOutputHeader,
}

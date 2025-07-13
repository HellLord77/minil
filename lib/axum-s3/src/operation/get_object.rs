use axum::body::Body;
use axum::extract::FromRequest;
use axum::extract::Path;
use axum_derive_macros::IntoResponse;
use axum_extra::extract::Query;
use axum_header::Header;
use bon::Builder;
use serde_s3::operation::GetObjectInputHeader;
use serde_s3::operation::GetObjectInputPath;
use serde_s3::operation::GetObjectInputQuery;
use serde_s3::operation::GetObjectOutputHeader;

#[derive(Debug, FromRequest)]
pub struct GetObjectInput {
    #[from_request(via(Path))]
    pub path: GetObjectInputPath,

    #[from_request(via(Query))]
    pub query: GetObjectInputQuery,

    #[from_request(via(Header))]
    pub header: GetObjectInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct GetObjectOutput {
    #[into_response(via(Header))]
    pub header: GetObjectOutputHeader,

    pub body: Body,
}

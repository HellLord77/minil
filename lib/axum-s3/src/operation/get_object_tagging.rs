use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_extra::extract::Query;
use axum_header::Header;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::GetObjectTaggingInputHeader;
use serde_s3::operation::GetObjectTaggingInputPath;
use serde_s3::operation::GetObjectTaggingInputQuery;
use serde_s3::operation::GetObjectTaggingOutputBody;
use serde_s3::operation::GetObjectTaggingOutputHeader;

#[derive(Debug, FromRequest)]
pub struct GetObjectTaggingInput {
    #[from_request(via(Path))]
    pub path: GetObjectTaggingInputPath,

    #[from_request(via(Query))]
    pub query: GetObjectTaggingInputQuery,

    #[from_request(via(Header))]
    pub header: GetObjectTaggingInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct GetObjectTaggingOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,

    #[into_response(via(Header))]
    pub header: GetObjectTaggingOutputHeader,

    #[into_response(via(Xml))]
    pub body: GetObjectTaggingOutputBody,
}

use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_extra::extract::Query;
use axum_header::Header;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::PutObjectTaggingInputBody;
use serde_s3::operation::PutObjectTaggingInputHeader;
use serde_s3::operation::PutObjectTaggingInputPath;
use serde_s3::operation::PutObjectTaggingInputQuery;
use serde_s3::operation::PutObjectTaggingOutputHeader;

#[derive(Debug, FromRequest)]
pub struct PutObjectTaggingInput {
    #[from_request(via(Path))]
    pub path: PutObjectTaggingInputPath,

    #[from_request(via(Query))]
    pub query: PutObjectTaggingInputQuery,

    #[from_request(via(Header))]
    pub header: PutObjectTaggingInputHeader,

    #[from_request(via(Xml))]
    pub body: PutObjectTaggingInputBody,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct PutObjectTaggingOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,

    #[into_response(via(Header))]
    pub header: PutObjectTaggingOutputHeader,
}

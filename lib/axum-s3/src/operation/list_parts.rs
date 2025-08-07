use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_extra::extract::Query;
use axum_header::Header;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::ListPartsInputHeader;
use serde_s3::operation::ListPartsInputPath;
use serde_s3::operation::ListPartsInputQuery;
use serde_s3::operation::ListPartsOutputBody;
use serde_s3::operation::ListPartsOutputHeader;

#[derive(Debug, FromRequest)]
pub struct ListPartsInput {
    #[from_request(via(Path))]
    pub path: ListPartsInputPath,

    #[from_request(via(Query))]
    pub query: ListPartsInputQuery,

    #[from_request(via(Header))]
    pub header: ListPartsInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct ListPartsOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,

    #[into_response(via(Header))]
    pub header: ListPartsOutputHeader,

    #[into_response(via(Xml))]
    pub body: ListPartsOutputBody,
}

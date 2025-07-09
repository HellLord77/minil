use axum::extract::FromRequest;
use axum_derive_macros::IntoResponse;
use axum_extra::extract::Query;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::ListBucketsInputQuery;
use serde_s3::operation::ListBucketsOutputBody;

#[derive(Debug, FromRequest)]
pub struct ListBucketsInput {
    #[from_request(via(Query))]
    pub query: ListBucketsInputQuery,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct ListBucketsOutput {
    #[into_response(via(Xml))]
    pub body: ListBucketsOutputBody,
}

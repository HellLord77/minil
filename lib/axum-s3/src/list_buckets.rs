use axum::extract::FromRequest;
use axum_extra::extract::Query;
use axum_into_response::IntoResponse;
use axum_xml::Xml;
use serde_s3::operation::ListBucketsInputQuery;
use serde_s3::operation::ListBucketsOutputBody;

#[derive(Debug, FromRequest)]
pub struct ListBucketsInput {
    #[from_request(via(Query))]
    pub query: ListBucketsInputQuery,
}

#[derive(Debug, IntoResponse)]
pub struct ListBucketsOutput {
    #[into_response(via(Xml))]
    pub body: ListBucketsOutputBody,
}

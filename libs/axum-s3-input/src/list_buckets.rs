use axum::extract::FromRequest;
use axum_extra::extract::Query;
use serde_s3::operation::ListBucketsInputQuery;

#[derive(Debug, FromRequest)]
pub struct ListBucketsInput {
    #[from_request(via(Query))]
    pub query: ListBucketsInputQuery,
}

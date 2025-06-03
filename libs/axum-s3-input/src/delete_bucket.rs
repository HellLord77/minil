use axum::extract::FromRequestParts;
use axum_extra::extract::Query;
use axum_header::Header;
use serde_s3::operation::DeleteBucketInputHeader;
use serde_s3::operation::DeleteBucketInputQuery;

#[derive(Debug, FromRequestParts)]
pub struct DeleteBucketInput {
    #[from_request(via(Query))]
    pub query: DeleteBucketInputQuery,

    #[from_request(via(Header))]
    pub header: DeleteBucketInputHeader,
}

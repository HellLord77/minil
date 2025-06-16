use axum::extract::FromRequest;
use axum::extract::Path;
use axum_extra::extract::Query;
use axum_header::Header;
use derive_getters::Getters;
use serde_s3::operation::DeleteBucketInputHeader;
use serde_s3::operation::DeleteBucketInputQuery;

#[derive(Debug, Getters, FromRequest)]
pub struct DeleteBucketInput {
    #[from_request(via(Path))]
    bucket: String,

    #[from_request(via(Query))]
    query: DeleteBucketInputQuery,

    #[from_request(via(Header))]
    header: DeleteBucketInputHeader,
}

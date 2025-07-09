use axum::extract::FromRequestParts;
use axum_derive_macros::OptionalFromRequestParts;
use axum_extra::extract::Query;
use serde_s3::utils::ListObjectsV2CheckQuery;

#[derive(Debug, FromRequestParts, OptionalFromRequestParts)]
pub struct ListObjectsV2Check {
    #[from_request(via(Query))]
    pub query: ListObjectsV2CheckQuery,
}

use axum::extract::FromRequestParts;
use axum_derive_macros::OptionalFromRequestPartsViaFromRequestParts;
use axum_extra::extract::Query;
use serde_s3::utils::GetBucketVersioningCheckQuery;

#[derive(Debug, FromRequestParts, OptionalFromRequestPartsViaFromRequestParts)]
pub struct GetBucketVersioningCheck {
    #[from_request(via(Query))]
    pub query: GetBucketVersioningCheckQuery,
}

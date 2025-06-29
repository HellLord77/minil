use axum::extract::FromRequestParts;
use axum_extra::extract::Query;
use axum_optional_from_request::OptionalFromRequestParts;
use serde_s3::utils::GetBucketVersioningCheckQuery;

#[derive(Debug, FromRequestParts, OptionalFromRequestParts)]
pub struct GetBucketVersioningCheck {
    #[from_request(via(Query))]
    pub query: GetBucketVersioningCheckQuery,
}

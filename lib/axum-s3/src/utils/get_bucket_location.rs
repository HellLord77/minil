use axum::extract::FromRequestParts;
use axum_extra::extract::Query;
use axum_optional_from_request::OptionalFromRequestParts;
use serde_s3::utils::GetBucketLocationCheckQuery;

#[derive(Debug, FromRequestParts, OptionalFromRequestParts)]
pub struct GetBucketLocationCheck {
    #[from_request(via(Query))]
    pub query: GetBucketLocationCheckQuery,
}

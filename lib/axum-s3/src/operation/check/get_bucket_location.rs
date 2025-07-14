use axum::extract::FromRequestParts;
use axum_derive_macros::OptionalFromRequestParts;
use axum_extra::extract::Query;
use serde_s3::operation::check::GetBucketLocationCheckQuery;

#[derive(Debug, FromRequestParts, OptionalFromRequestParts)]
pub struct GetBucketLocationCheck {
    #[from_request(via(Query))]
    pub query: GetBucketLocationCheckQuery,
}

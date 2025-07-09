use axum::extract::FromRequestParts;
use axum_derive_macros::OptionalFromRequestPartsViaFromRequestParts;
use axum_extra::extract::Query;
use serde_s3::utils::GetBucketLocationCheckQuery;

#[derive(Debug, FromRequestParts, OptionalFromRequestPartsViaFromRequestParts)]
pub struct GetBucketLocationCheck {
    #[from_request(via(Query))]
    pub query: GetBucketLocationCheckQuery,
}

use axum::extract::FromRequestParts;
use axum_extra::extract::Query;
use axum_optional_from_request::OptionalFromRequestParts;
use serde_s3::utils::ListObjectsV2CheckQuery;

#[derive(Debug, FromRequestParts, OptionalFromRequestParts)]
pub struct ListObjectsV2Check(
    #[allow(dead_code)]
    #[from_request(via(Query))]
    ListObjectsV2CheckQuery,
);

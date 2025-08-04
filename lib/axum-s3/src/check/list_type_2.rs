use axum::extract::FromRequestParts;
use axum_derive_macros::OptionalFromRequestParts;
use axum_extra::extract::Query;
use serde_s3::check::ListType2CheckQuery;

#[derive(Debug, FromRequestParts, OptionalFromRequestParts)]
pub struct ListType2Check {
    #[from_request(via(Query))]
    pub query: ListType2CheckQuery,
}

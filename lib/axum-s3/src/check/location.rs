use axum::extract::FromRequestParts;
use axum_derive_macros::OptionalFromRequestParts;
use axum_extra::extract::Query;
use serde_s3::check::LocationCheckQuery;

#[derive(Debug, FromRequestParts, OptionalFromRequestParts)]
pub struct LocationCheck {
    #[from_request(via(Query))]
    pub query: LocationCheckQuery,
}

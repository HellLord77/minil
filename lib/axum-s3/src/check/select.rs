use axum::extract::FromRequestParts;
use axum_derive_macros::OptionalFromRequestParts;
use axum_extra::extract::Query;
use serde_s3::check::SelectCheckQuery;

#[derive(Debug, FromRequestParts, OptionalFromRequestParts)]
pub struct SelectCheck {
    #[from_request(via(Query))]
    pub query: SelectCheckQuery,
}

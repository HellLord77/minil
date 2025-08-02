use axum::extract::FromRequestParts;
use axum_derive_macros::OptionalFromRequestParts;
use axum_extra::extract::Query;
use serde_s3::operation::check::SelectObjectContentCheckQuery;

#[derive(Debug, FromRequestParts, OptionalFromRequestParts)]
pub struct SelectObjectContentCheck {
    #[from_request(via(Query))]
    pub query: SelectObjectContentCheckQuery,
}

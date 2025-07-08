use axum_core::response::IntoResponse;
use derive_more::Display;
use derive_more::Error;

#[derive(Debug, Display, Error)]
#[error(ignore)]
pub enum RejectionError<L, R>
where
    L: IntoResponse,
    R: IntoResponse,
{
    LeftRejection(L),
    RightRejection(R),
}

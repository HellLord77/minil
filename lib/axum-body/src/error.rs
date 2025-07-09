use axum_core::response::IntoResponse;
use derive_more::Constructor;
use derive_more::Display;
use derive_more::Error;

#[derive(Debug, Display, Constructor, Error)]
#[display("{kind} rejection: {rejection}")]
pub struct RejectionError<T> {
    rejection: T,
    kind: RejectionErrorKind,
}

impl<T> RejectionError<T>
where
    T: IntoResponse,
{
    pub fn left(rejection: T) -> Self {
        Self::new(rejection, RejectionErrorKind::Left)
    }

    pub fn right(rejection: T) -> Self {
        Self::new(rejection, RejectionErrorKind::Right)
    }
}

#[derive(Debug, Display)]
pub enum RejectionErrorKind {
    Left,
    Right,
}

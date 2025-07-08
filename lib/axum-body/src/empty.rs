use std::fmt::Debug;
use std::fmt::Display;

use axum_core::extract::FromRequest;
use axum_core::extract::Request;

use crate::RejectionError;
use crate::rejection::EmptyRejection;
use crate::rejection::NonEmptyRejectionError;
use crate::rejection::NotEmptyRejection;
use crate::rejection::UnknownBodyError;
use crate::utils::has_remaining;

#[derive(Debug)]
#[must_use]
pub struct Empty;

impl<S> FromRequest<S> for Empty
where
    S: Send + Sync,
{
    type Rejection = EmptyRejection;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let body = req.into_body();
        if has_remaining(body)
            .await
            .map_err(UnknownBodyError::from_err)?
            .0
        {
            Err(NotEmptyRejection)?
        } else {
            Ok(Self)
        }
    }
}

#[derive(Debug)]
#[must_use]
pub struct EmptyOr<T>(pub Option<T>);

impl<T, S> FromRequest<S> for EmptyOr<T>
where
    T: Send + Sync + FromRequest<S>,
    S: Send + Sync,
    T::Rejection: 'static + Debug + Display + Send + Sync,
{
    type Rejection = EmptyRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        match has_remaining(body)
            .await
            .map_err(UnknownBodyError::from_err)?
        {
            (true, body) => {
                let req = Request::from_parts(parts, body);
                match T::from_request(req, state).await {
                    Ok(data) => Ok(Self(Some(data))),
                    Err(rej) => Err(NonEmptyRejectionError::from_err(
                        RejectionError::RightRejection::<T::Rejection, _>(rej),
                    ))?,
                }
            }
            (false, _) => Ok(Self(None)),
        }
    }
}

use std::fmt::Debug;
use std::fmt::Display;
use std::pin::pin;

use axum_core::body::Body;
use axum_core::extract::FromRequest;
use axum_core::extract::Request;
use futures::StreamExt;
use futures::TryStreamExt;

use crate::error::RejectionError;
use crate::rejection::EmptyRejection;
use crate::rejection::NonEmptyRejectionError;
use crate::rejection::NotEmptyRejection;
use crate::rejection::UnknownBodyError;

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
        let mut stream = pin!(body.into_data_stream());

        let is_empty = match stream
            .try_next()
            .await
            .map_err(UnknownBodyError::from_err)?
        {
            Some(chunk) => chunk.is_empty(),
            None => true,
        };

        Ok(is_empty.then_some(Self).ok_or(NotEmptyRejection)?)
    }
}

#[derive(Debug)]
#[must_use]
pub struct OptionalEmpty<T>(pub Option<T>);

impl<T, S> FromRequest<S> for OptionalEmpty<T>
where
    T: Send + Sync + FromRequest<S>,
    S: Send + Sync,
    T::Rejection: 'static + Debug + Display + Send + Sync,
{
    type Rejection = EmptyRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let mut stream = Box::pin(body.into_data_stream().peekable());

        let is_empty = match stream.as_mut().peek().await {
            Some(Ok(chunk)) => chunk.is_empty(),
            Some(Err(_)) => {
                stream
                    .try_next()
                    .await
                    .map_err(UnknownBodyError::from_err)?;
                unreachable!()
            }
            None => true,
        };

        if is_empty {
            Ok(Self(None))
        } else {
            let body = Body::from_stream(stream);
            let req = Request::from_parts(parts, body);

            match T::from_request(req, state).await {
                Ok(data) => Ok(Self(Some(data))),
                Err(err) => Err(NonEmptyRejectionError::from_err(RejectionError::right(err)))?,
            }
        }
    }
}

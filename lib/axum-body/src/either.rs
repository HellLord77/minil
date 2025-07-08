use std::fmt::Debug;
use std::fmt::Display;

use axum_core::extract::FromRequest;
use axum_core::extract::Request;

use crate::error::RejectionError;
use crate::rejection::EitherRejection;
use crate::rejection::EitherRejectionError;
use crate::utils::cloned2;

#[derive(Debug)]
#[must_use]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn unwrap_left(self) -> L {
        match self {
            Self::Left(data) => data,
            Self::Right(_) => {
                panic!("Cannot unwrap Left branch. Either contains an `R` type.")
            }
        }
    }

    pub fn unwrap_right(self) -> R {
        match self {
            Self::Left(_) => {
                panic!("Cannot unwrap Right branch. Either contains an `L` type.")
            }
            Self::Right(data) => data,
        }
    }
}

impl<L, R, S> FromRequest<S> for Either<L, R>
where
    L: Send + Sync + FromRequest<S>,
    R: Send + Sync + FromRequest<S>,
    S: Send + Sync,
    L::Rejection: 'static + Debug + Display + Send + Sync,
    R::Rejection: 'static + Debug + Display + Send + Sync,
{
    type Rejection = EitherRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (req1, req2) = cloned2(req, state).await?;
        match L::from_request(req1, state).await {
            Ok(data) => Ok(Either::Left(data)),
            Err(rej) => match R::from_request(req2, state).await {
                Ok(data) => Ok(Either::Right(data)),
                Err(_) => {
                    let err = RejectionError::LeftRejection::<_, R::Rejection>(rej);
                    Err(EitherRejectionError::from_err(err))?
                }
            },
        }
    }
}

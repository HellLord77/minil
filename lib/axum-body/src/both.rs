use std::fmt::Debug;
use std::fmt::Display;

use axum_core::extract::FromRequest;
use axum_core::extract::Request;

use crate::error::RejectionError;
use crate::rejection::BothRejection;
use crate::rejection::BothRejectionError;
use crate::utils::cloned2;

#[derive(Debug)]
#[must_use]
pub struct Both<L, R>(pub L, pub R);

impl<L, R, S> FromRequest<S> for Both<L, R>
where
    L: Send + Sync + FromRequest<S>,
    R: Send + Sync + FromRequest<S>,
    S: Send + Sync,
    L::Rejection: 'static + Debug + Display + Send + Sync,
    R::Rejection: 'static + Debug + Display + Send + Sync,
{
    type Rejection = BothRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (req1, req2) = cloned2(req, state).await?;

        Ok(Both(
            L::from_request(req1, state)
                .await
                .map_err(|rej| BothRejectionError::from_err(RejectionError::left(rej)))?,
            R::from_request(req2, state)
                .await
                .map_err(|rej| BothRejectionError::from_err(RejectionError::right(rej)))?,
        ))
    }
}

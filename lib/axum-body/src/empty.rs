use axum_core::extract::FromRequest;
use axum_core::extract::Request;
use http_body_util::BodyExt;
use http_body_util::Limited;

use crate::rejection::EmptyRejection;
use crate::rejection::LimitedBodyError;

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
        Limited::new(body, 0)
            .collect()
            .await
            .map_err(LimitedBodyError::from_err)?;

        Ok(Empty)
    }
}

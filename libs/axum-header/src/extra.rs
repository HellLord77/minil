use crate::Header;
use axum_core::extract::FromRequestParts;
use http::request::Parts;
use serde::de::DeserializeOwned;
use std::convert::Infallible;
use std::ops;

#[derive(Debug, Clone, Copy, Default)]
pub struct OptionalHeader<T>(pub Option<T>);

impl<T, S> FromRequestParts<S> for OptionalHeader<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let values = Header::from_request_parts(parts, _state)
            .await
            .map(|header| header.0)
            .ok();
        Ok(OptionalHeader(values))
    }
}

impl<T> ops::Deref for OptionalHeader<T> {
    type Target = Option<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for OptionalHeader<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

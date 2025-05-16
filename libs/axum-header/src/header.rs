use crate::{rejection::FailedToDeserializeHeaderString, rejection::HeaderRejection};
use axum_core::extract::FromRequestParts;
use http::{HeaderMap, request::Parts};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Copy, Default)]
pub struct Header<T>(pub T);

impl<T, S> FromRequestParts<S> for Header<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = HeaderRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Self::try_from_header(&parts.headers)
    }
}

impl<T> Header<T>
where
    T: DeserializeOwned,
{
    pub fn try_from_header(value: &HeaderMap) -> Result<Self, HeaderRejection> {
        let header = form_urlencoded::Serializer::new(String::new())
            .extend_pairs(
                value
                    .into_iter()
                    .map(|(name, value)| (name, value.to_str().unwrap_or_default())),
            )
            .finish();
        let deserializer =
            serde_urlencoded::Deserializer::new(form_urlencoded::parse(header.as_bytes()));
        let values = serde_path_to_error::deserialize(deserializer)
            .map_err(FailedToDeserializeHeaderString::from_err)?;
        Ok(Header(values))
    }
}

axum_core::__impl_deref!(Header);

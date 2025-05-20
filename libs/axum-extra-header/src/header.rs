use crate::HeaderRejection;
use crate::rejection::FailedToDeserializeHeaderString;
use crate::rejection::OptionalHeaderRejection;
use axum_core::extract::FromRequestParts;
use http::request::Parts;
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
        let header = form_urlencoded::Serializer::new(String::new())
            .extend_pairs(
                parts
                    .headers
                    .iter()
                    .map(|(name, value)| (name, value.to_str().unwrap())),
            )
            .finish();
        let deserializer =
            serde_html_form::Deserializer::new(form_urlencoded::parse(header.as_bytes()));
        let values = serde_path_to_error::deserialize(deserializer)
            .map_err(FailedToDeserializeHeaderString::from_err)?;
        Ok(Header(values))
    }
}

axum_core::__impl_deref!(Header);

#[derive(Debug, Clone, Copy, Default)]
pub struct OptionalHeader<T>(pub Option<T>);

impl<T, S> FromRequestParts<S> for OptionalHeader<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = OptionalHeaderRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if !parts.headers.is_empty() {
            let header = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(
                    parts
                        .headers
                        .iter()
                        .map(|(name, value)| (name, value.to_str().unwrap())),
                )
                .finish();
            let deserializer =
                serde_html_form::Deserializer::new(form_urlencoded::parse(header.as_bytes()));
            let values = serde_path_to_error::deserialize(deserializer)
                .map_err(FailedToDeserializeHeaderString::from_err)?;
            Ok(OptionalHeader(Some(values)))
        } else {
            Ok(OptionalHeader(None))
        }
    }
}

impl<T> std::ops::Deref for OptionalHeader<T> {
    type Target = Option<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for OptionalHeader<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

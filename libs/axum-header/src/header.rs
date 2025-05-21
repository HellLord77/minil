use crate::HeaderRejection;
use crate::rejection::FailedToDeserializeHeaderString;
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
                    .map(|(name, value)| (name, value.to_str().unwrap_or_else(|_err| todo!()))),
            )
            .finish();

        let parser = form_urlencoded::parse(header.as_bytes());
        #[cfg(not(feature = "extra"))]
        let deserializer = serde_urlencoded::Deserializer::new(parser);
        #[cfg(feature = "extra")]
        let deserializer = serde_html_form::Deserializer::new(parser);

        let values = serde_path_to_error::deserialize(deserializer)
            .map_err(FailedToDeserializeHeaderString::from_err)?;
        Ok(Header(values))
    }
}

axum_core::__impl_deref!(Header);

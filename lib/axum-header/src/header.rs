use axum_core::extract::FromRequestParts;
use axum_core::response::IntoResponse;
use axum_core::response::IntoResponseParts;
use axum_core::response::Response;
use axum_core::response::ResponseParts;
use http::HeaderName;
use http::request::Parts;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::HeaderError;
use crate::HeaderRejection;
use crate::rejection::FailedToDeserializeHeaderString;

#[derive(Debug, Clone, Copy, Default)]
pub struct Header<T>(pub T);

impl<T, S> FromRequestParts<S> for Header<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = HeaderRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let deserializer = serde_header::Deserializer::from_header_map(&parts.headers);
        let value = serde_path_to_error::deserialize(deserializer)
            .map_err(FailedToDeserializeHeaderString::from_err)?;

        Ok(Header(value))
    }
}

axum_core::__impl_deref!(Header);

impl<T> From<T> for Header<T> {
    fn from(inner: T) -> Self {
        Header(inner)
    }
}

impl<T> IntoResponseParts for Header<T>
where
    T: Serialize,
{
    type Error = HeaderError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        let mut value = vec![];
        let serializer = serde_header::Serializer::new(&mut value);
        serde_path_to_error::serialize(&self.0, serializer)?;

        let headers = res.headers_mut();
        headers.try_reserve(value.len())?;
        for (name, value) in value {
            headers.append(name.parse::<HeaderName>()?, value.try_into()?);
        }

        Ok(res)
    }
}

impl<T> IntoResponse for Header<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (self, ()).into_response()
    }
}

use crate::HeaderRejection;
use crate::TryIntoHeaderError;
use crate::rejection::FailedToDeserializeHeaderString;
use axum_core::extract::FromRequestParts;
use axum_core::response::IntoResponse;
use axum_core::response::IntoResponseParts;
use axum_core::response::Response;
use axum_core::response::ResponseParts;
use http::HeaderName;
use http::HeaderValue;
use http::request::Parts;
use serde::Serialize;
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
        let encoded =
            form_urlencoded::Serializer::new(String::new())
                .extend_pairs(parts.headers.iter().map(|(key, value)| {
                    (key, value.to_str().unwrap_or_else(|_err| unimplemented!()))
                }))
                .finish();

        let parser = form_urlencoded::parse(encoded.as_bytes());
        #[cfg(not(feature = "extra"))]
        let deserializer = serde_urlencoded::Deserializer::new(parser);
        #[cfg(feature = "extra")]
        let deserializer = serde_html_form::Deserializer::new(parser);

        Ok(Header(
            serde_path_to_error::deserialize(deserializer)
                .map_err(FailedToDeserializeHeaderString::from_err)?,
        ))
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
    type Error = TryIntoHeaderError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        #[cfg(not(feature = "extra"))]
        let serializer = serde_urlencoded::Serializer::new(&mut encoder);
        #[cfg(feature = "extra")]
        let serializer = serde_html_form::Serializer::new(&mut encoder);

        self.0
            .serialize(serializer)
            .unwrap_or_else(|_err| unimplemented!());
        let encoded = encoder.finish();

        let parser = form_urlencoded::parse(encoded.as_bytes());
        for (key, value) in parser {
            res.headers_mut().append(
                key.parse::<HeaderName>()
                    .map_err(|_err| TryIntoHeaderError::from_name(key.into_owned()))?,
                value
                    .parse::<HeaderValue>()
                    .map_err(|_err| TryIntoHeaderError::from_value(value.into_owned()))?,
            );
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

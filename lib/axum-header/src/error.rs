use axum_core::response::IntoResponse;
use axum_core::response::Response;
use derive_more::Constructor;
use derive_more::Display;
use derive_more::Error;
use http::StatusCode;

#[derive(Debug, Display)]
pub enum TryIntoHeaderErrorKind {
    Name,
    Value,
}

#[derive(Debug, Display, Constructor, Error)]
#[display("failed to convert `{unknown}` to header {}", kind.to_string().to_lowercase())]
pub struct TryIntoHeaderError {
    unknown: String,
    kind: TryIntoHeaderErrorKind,
}

impl TryIntoHeaderError {
    #[inline]
    pub fn from_name(unknown: String) -> Self {
        Self::new(unknown, TryIntoHeaderErrorKind::Name)
    }

    #[inline]
    pub fn from_value(unknown: String) -> Self {
        Self::new(unknown, TryIntoHeaderErrorKind::Value)
    }
}

impl IntoResponse for TryIntoHeaderError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

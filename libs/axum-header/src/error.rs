use axum_core::response::IntoResponse;
use axum_core::response::Response;
use http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("failed to convert `{0}` to a header {kind}", kind = if matches!(self, Self::Name(_)) { "name" } else { "value" })]
pub enum TryIntoHeaderError {
    Name(String),
    Value(String),
}

impl IntoResponse for TryIntoHeaderError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

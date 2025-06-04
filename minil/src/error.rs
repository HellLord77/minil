use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use std::result;

pub(crate) struct Error(anyhow::Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
    }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub(crate) type Result<T> = result::Result<T, Error>;

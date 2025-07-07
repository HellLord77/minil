use axum_core::response::IntoResponse;
use axum_core::response::Response;
use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use http::StatusCode;
use http::header::InvalidHeaderName;
use http::header::InvalidHeaderValue;
use http::header::MaxSizeReached;

#[derive(Debug, Display, From, Error)]
#[display("Failed to serialize header: {_0}")]
pub enum HeaderError {
    Serialization(serde_path_to_error::Error<serde_header::ser::error::Error>),
    Size(MaxSizeReached),
    Name(InvalidHeaderName),
    Value(InvalidHeaderValue),
}

impl IntoResponse for HeaderError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

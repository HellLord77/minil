use axum::http::request::Parts;
use bon::Builder;

#[derive(Debug, Builder)]
pub struct ErrorParts {
    #[builder(into)]
    pub resource: String,

    #[builder(into)]
    pub request_id: Option<String>,
}

impl From<&Parts> for ErrorParts {
    fn from(parts: &Parts) -> Self {
        let resource = parts.uri.path();
        let maybe_request_id = parts
            .headers
            .get("x-amz-request-id")
            .and_then(|value| value.to_str().ok());

        Self::builder()
            .resource(resource)
            .maybe_request_id(maybe_request_id)
            .build()
    }
}

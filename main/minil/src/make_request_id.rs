use axum::http::Request;
use tower_http::request_id::MakeRequestId;
use tower_http::request_id::RequestId;
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct AppMakeRequestId;

impl MakeRequestId for AppMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        Some(RequestId::new(Uuid::new_v4().to_string().parse().unwrap()))
    }
}

use std::time::SystemTime;

use axum::http::Request;
use tower_http::request_id::MakeRequestId;
use tower_http::request_id::RequestId;

#[derive(Clone)]
pub(crate) struct AppMakeRequestId;

impl MakeRequestId for AppMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let request_id = format!("{nanos:X}");
        Some(RequestId::new(request_id.parse().unwrap()))
    }
}

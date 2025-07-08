use axum_body::Empty;
use axum_core::body::Body;
use axum_core::extract::FromRequest;
use axum_core::extract::Request;

#[tokio::test]
async fn test_empty() {
    let req = Request::builder().body(Body::empty()).unwrap();

    let empty = Empty::from_request(req, &()).await;
    assert!(empty.is_ok());
}

#[tokio::test]
async fn test_empty_rejection() {
    let req = Request::builder().body(Body::from("foo")).unwrap();

    let empty = Empty::from_request(req, &()).await;
    assert!(empty.is_err());
}

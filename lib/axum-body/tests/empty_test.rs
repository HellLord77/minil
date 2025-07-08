use axum_body::Empty;
use axum_body::OptionalEmpty;
use axum_core::body::Body;
use axum_core::extract::FromRequest;
use axum_core::extract::Request;
use bytes::Bytes;
use http::StatusCode;

#[tokio::test]
async fn test_empty() {
    let req = Request::builder().body(Body::empty()).unwrap();

    let empty = Empty::from_request(req, &()).await.unwrap();
    assert!(matches!(&empty, Empty));
}

#[tokio::test]
async fn test_empty_rejection() {
    let req = Request::builder().body(Body::from("'")).unwrap();

    let rejection = Empty::from_request(req, &()).await.unwrap_err();
    assert_eq!(&rejection.status(), &StatusCode::BAD_REQUEST);
    assert_eq!(&rejection.body_text(), "Expected request with empty body");
}

#[tokio::test]
async fn test_optional_empty_none() {
    let req = Request::builder().body(Body::empty()).unwrap();

    let bytes = OptionalEmpty::<Bytes>::from_request(req, &())
        .await
        .unwrap()
        .0;
    assert_eq!(&bytes, &None);
}

#[tokio::test]
async fn test_optional_empty_some() {
    let req = Request::builder().body(Body::from("'")).unwrap();

    let bytes = OptionalEmpty::<Bytes>::from_request(req, &())
        .await
        .unwrap()
        .0
        .unwrap();
    assert_eq!(&bytes, "'");
}

#[tokio::test]
async fn test_optional_empty_rejection() {
    let req = Request::builder().body(Body::from(vec![0x80])).unwrap();

    let rejection = OptionalEmpty::<String>::from_request(req, &())
        .await
        .unwrap_err();
    assert_eq!(&rejection.status(), &StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        &rejection.to_string(),
        "Failed to process non empty entity: Request body didn't contain valid UTF-8: invalid utf-8 sequence of 1 bytes from index 0"
    );
}

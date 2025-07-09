use axum::Form;
use axum::Json;
use axum_body::Both;
use axum_core::body::Body;
use axum_core::extract::FromRequest;
use axum_core::extract::Request;
use bytes::Bytes;
use http::StatusCode;
use serde::Deserialize;
use serde::Serialize;

mod utils;
use utils::set_form;
use utils::set_json;

#[derive(Debug, Serialize, Deserialize)]
struct TestForm {
    hello: String,
}

#[tokio::test]
async fn test_both_extract_left() {
    let req = set_form(
        Request::builder(),
        TestForm {
            hello: "world".to_owned(),
        },
    );

    let both = Both::<Form<TestForm>, Bytes>::from_request(req, &())
        .await
        .unwrap();
    assert_eq!(&both.0.hello, "world");
    assert_eq!(&both.1, "hello=world");
}

#[tokio::test]
async fn test_both_extract_right() {
    let req = set_json(
        Request::builder(),
        TestForm {
            hello: "world".to_owned(),
        },
    );

    let both = Both::<Bytes, Json<TestForm>>::from_request(req, &())
        .await
        .unwrap();
    assert_eq!(&both.0, "{\"hello\":\"world\"}");
    assert_eq!(&both.1.hello, "world");
}

#[tokio::test]
async fn test_both_extract_recursive() {
    let req = set_form(
        Request::builder(),
        TestForm {
            hello: "world".to_owned(),
        },
    );

    let both = Both::<Both<Bytes, String>, Form<TestForm>>::from_request(req, &())
        .await
        .unwrap();
    assert_eq!(&both.0.0, "hello=world");
    assert_eq!(&both.0.1, "hello=world");
    assert_eq!(&both.1.0.hello, "world");
}

#[tokio::test]
async fn test_both_rejection_left() {
    let req = Request::builder().body(Body::empty()).unwrap();

    let rejection = Both::<Form<TestForm>, Bytes>::from_request(req, &())
        .await
        .unwrap_err();
    assert_eq!(&rejection.status(), &StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        &rejection.body_text(),
        "Failed to process either entity: Left rejection: Failed to deserialize form: missing field `hello`"
    );
}

#[tokio::test]
async fn test_both_rejection_right() {
    let req = Request::builder().body(Body::empty()).unwrap();

    let rejection = Both::<Bytes, Form<TestForm>>::from_request(req, &())
        .await
        .unwrap_err();
    assert_eq!(&rejection.status(), &StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        &rejection.body_text(),
        "Failed to process either entity: Right rejection: Failed to deserialize form: missing field `hello`"
    );
}

use axum::Form;
use axum::Json;
use axum_body::Either;
use axum_core::body::Body;
use axum_core::extract::FromRequest;
use axum_core::extract::Request;
use bytes::Bytes;
use http::Method;
use http::StatusCode;
use http::header;
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
async fn test_either_extract_first_try() {
    let req = set_form(
        Request::builder(),
        TestForm {
            hello: "world".to_owned(),
        },
    );

    let form = Either::<Form<TestForm>, Json<TestForm>>::from_request(req, &())
        .await
        .unwrap()
        .unwrap_left()
        .0;
    assert_eq!(&form.hello, "world");
}

#[tokio::test]
async fn test_either_extract_fallback() {
    let req = set_json(
        Request::builder(),
        TestForm {
            hello: "world".to_owned(),
        },
    );

    let form = Either::<Form<TestForm>, Json<TestForm>>::from_request(req, &())
        .await
        .unwrap()
        .unwrap_right()
        .0;
    assert_eq!(&form.hello, "world");
}

#[tokio::test]
async fn test_either_extract_recursive_fallback() {
    let req = Request::builder()
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(b"!@$%^&*()".as_ref()))
        .unwrap();

    let payload = Either::<Either<Form<TestForm>, Json<TestForm>>, Bytes>::from_request(req, &())
        .await
        .unwrap()
        .unwrap_right();
    assert_eq!(&payload.as_ref(), &b"!@$%^&*()");
}

#[tokio::test]
async fn test_either_rejection() {
    let req = Request::builder().body(Body::empty()).unwrap();

    let rejection = Either::<Form<TestForm>, Json<TestForm>>::from_request(req, &())
        .await
        .unwrap_err();
    assert_eq!(&rejection.status(), &StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        &rejection.body_text(),
        "Failed to process both entities: Left rejection: Failed to deserialize form: missing field `hello`"
    );
}

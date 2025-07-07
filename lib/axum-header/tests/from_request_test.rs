use std::fmt::Debug;

use axum::Router;
use axum::routing::get;
use axum::routing::post;
use axum_core::body::Body;
use axum_core::extract::FromRequest;
use axum_header::Header;
use axum_test::TestServer;
use http::HeaderMap;
use http::HeaderName;
use http::HeaderValue;
use http::Request;
use http::StatusCode;
use serde::Deserialize;
use serde::de::DeserializeOwned;

#[tokio::test]
async fn test_header() {
    async fn check<T>(header: HeaderMap, value: T)
    where
        T: DeserializeOwned + PartialEq + Debug,
    {
        let mut req = Request::builder().body(Body::empty()).unwrap();
        req.headers_mut().extend(header);
        assert_eq!(Header::<T>::from_request(req, &()).await.unwrap().0, value);
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct Pagination {
        size: Option<u64>,
        page: Option<u64>,
    }

    check(
        HeaderMap::new(),
        Pagination {
            size: None,
            page: None,
        },
    )
    .await;

    check(
        HeaderMap::from_iter([(
            HeaderName::from_static("size"),
            HeaderValue::from_static("10"),
        )]),
        Pagination {
            size: Some(10),
            page: None,
        },
    )
    .await;

    check(
        HeaderMap::from_iter([
            (
                HeaderName::from_static("size"),
                HeaderValue::from_static("10"),
            ),
            (
                HeaderName::from_static("page"),
                HeaderValue::from_static("20"),
            ),
        ]),
        Pagination {
            size: Some(10),
            page: Some(20),
        },
    )
    .await;
}

#[tokio::test]
async fn correct_rejection_status_code() {
    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Values {
        n: i32,
    }

    async fn handler(_: Header<Values>) {}

    let app = Router::new().route("/", get(handler));
    let server = TestServer::new(app).unwrap();

    let res = server.get("/").add_header("n", "hi").await;
    assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
    assert_eq!(
        res.text(),
        "Failed to deserialize header string: n: invalid digit found in string"
    );
}

#[tokio::test]
async fn header_supports_multiple_values() {
    #[derive(Deserialize)]
    struct Data {
        #[serde(rename = "value")]
        values: Vec<String>,
    }

    let app = Router::new().route(
        "/",
        post(|Header(data): Header<Data>| async move { data.values.join(",") }),
    );

    let server = TestServer::new(app).unwrap();

    let res = server
        .post("/")
        .add_header("value", "one")
        .add_header("value", "two")
        .await;

    assert_eq!(res.status_code(), StatusCode::OK);
    assert_eq!(res.text(), "one,two");
}

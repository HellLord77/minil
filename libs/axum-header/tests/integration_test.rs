use axum::Router;
use axum::routing::get;
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
use std::fmt::Debug;

async fn check<T>(header: HeaderMap, value: T)
where
    T: DeserializeOwned + PartialEq + Debug,
{
    let mut req = Request::builder().body(Body::empty()).unwrap();
    req.headers_mut().extend(header);
    assert_eq!(Header::<T>::from_request(req, &()).await.unwrap().0, value);
}

#[tokio::test]
async fn test_header() {
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

#[test]
fn test_try_from_header() {
    #[derive(Deserialize)]
    struct TestHeaderValues {
        foo: String,
        bar: u32,
    }
    let header = HeaderMap::from_iter([
        (
            HeaderName::from_static("foo"),
            HeaderValue::from_static("hello"),
        ),
        (
            HeaderName::from_static("bar"),
            HeaderValue::from_static("42"),
        ),
    ]);
    let result: Header<TestHeaderValues> = Header::try_from_header(&header).unwrap();
    assert_eq!(result.foo, String::from("hello"));
    assert_eq!(result.bar, 42);
}

#[test]
fn test_try_from_header_with_invalid_value() {
    #[derive(Deserialize)]
    struct TestHeaderValues {
        _foo: String,
        _bar: u32,
    }
    let header = HeaderMap::from_iter([
        (
            HeaderName::from_static("foo"),
            HeaderValue::from_static("hello"),
        ),
        (
            HeaderName::from_static("bar"),
            HeaderValue::from_static("invalid"),
        ),
    ]);
    let result: Result<Header<TestHeaderValues>, _> = Header::try_from_header(&header);

    assert!(result.is_err());
}

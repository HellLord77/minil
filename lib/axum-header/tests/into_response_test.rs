use std::fmt::Debug;

use axum::Router;
use axum::routing::get;
use axum::routing::post;
use axum_core::response::IntoResponse;
use axum_header::Header;
use axum_test::TestServer;
use http::HeaderMap;
use http::HeaderName;
use http::HeaderValue;
use http::StatusCode;
use itertools::Itertools;
use serde::Serialize;

#[tokio::test]
async fn test_header() {
    async fn check<T>(value: T, header: HeaderMap)
    where
        T: Serialize + PartialEq + Debug,
    {
        let res = Header(value).into_response();
        assert_eq!(res.headers(), &header);
    }

    #[derive(Debug, PartialEq, Serialize)]
    struct Pagination {
        size: Option<u64>,
        page: Option<u64>,
    }

    check(
        Pagination {
            size: None,
            page: None,
        },
        HeaderMap::new(),
    )
    .await;

    check(
        Pagination {
            size: Some(10),
            page: None,
        },
        HeaderMap::from_iter([(
            HeaderName::from_static("size"),
            HeaderValue::from_static("10"),
        )]),
    )
    .await;

    check(
        Pagination {
            size: Some(10),
            page: Some(20),
        },
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
    )
    .await;
}

#[tokio::test]
async fn correct_error_status_code() {
    #[derive(Serialize)]
    #[allow(dead_code)]
    struct Values {
        n: Vec<i32>,
    }

    async fn handler() -> Header<Values> {
        Header(Values { n: vec![0; 100000] })
    }

    let app = Router::new().route("/", get(handler));
    let server = TestServer::new(app).unwrap();

    let res = server.get("/").await;
    assert_eq!(res.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(res.text(), "Failed to serialize header: max size reached");
}

#[tokio::test]
async fn header_supports_multiple_values() {
    #[derive(Serialize)]
    struct Data {
        #[serde(rename = "value")]
        values: Vec<String>,
    }

    let app = Router::new().route(
        "/",
        post(async || {
            Header(Data {
                values: vec!["one".to_owned(), "two".to_owned()],
            })
        }),
    );

    let server = TestServer::new(app).unwrap();

    let res = server.post("/").await;

    assert_eq!(res.status_code(), StatusCode::OK);
    assert_eq!(
        res.headers()
            .get_all("value")
            .iter()
            .map(|v| v.to_str().unwrap())
            .join(","),
        "one,two"
    );
}

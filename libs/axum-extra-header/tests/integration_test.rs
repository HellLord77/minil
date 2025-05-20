use axum::Router;
use axum::routing::get;
use axum::routing::post;
use axum_extra_header::Header;
use axum_extra_header::OptionalHeader;
use axum_test::TestServer;
use http::StatusCode;
use serde::Deserialize;

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
async fn optional_header_supports_multiple_values() {
    #[derive(Deserialize)]
    struct Data {
        #[serde(rename = "value")]
        values: Vec<String>,
    }

    let app = Router::new().route(
        "/",
        post(|OptionalHeader(data): OptionalHeader<Data>| async move {
            data.map(|Data { values }| values.join(","))
                .unwrap_or("None".to_owned())
        }),
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

#[tokio::test]
async fn optional_header_deserializes_no_values_into_none() {
    #[derive(Deserialize)]
    struct Data {
        value: String,
    }

    let app = Router::new().route(
        "/",
        post(|OptionalHeader(data): OptionalHeader<Data>| async move {
            match data {
                None => "None".into(),
                Some(data) => data.value,
            }
        }),
    );

    let server = TestServer::new(app).unwrap();

    let res = server.post("/").await;

    assert_eq!(res.status_code(), StatusCode::OK);
    assert_eq!(res.text(), "None");
}

#[tokio::test]
async fn optional_header_preserves_parsing_errors() {
    #[derive(Deserialize)]
    struct Data {
        value: String,
    }

    let app = Router::new().route(
        "/",
        post(|OptionalHeader(data): OptionalHeader<Data>| async move {
            match data {
                None => "None".into(),
                Some(data) => data.value,
            }
        }),
    );

    let server = TestServer::new(app).unwrap();

    let res = server.post("/").add_header("other", "something").await;

    assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

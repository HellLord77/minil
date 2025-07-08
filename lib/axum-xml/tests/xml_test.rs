use axum::Router;
use axum::routing::post;
use axum_test::TestServer;
use axum_xml::Xml;
use http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Value;

#[tokio::test]
async fn deserialize_body() {
    #[derive(Debug, Deserialize)]
    struct Input {
        foo: String,
    }

    let app = Router::new().route("/", post(|input: Xml<Input>| async { input.0.foo }));

    let server = TestServer::new(app).unwrap();
    let res = server
        .post("/")
        .text(r#"<Input><foo>bar</foo></Input>"#)
        .content_type("application/xml")
        .await;
    let body = res.text();

    assert_eq!(body, "bar");
}

#[tokio::test]
async fn consume_xml_to_xml_requires_xml_content_type() {
    #[derive(Debug, Deserialize)]
    struct Input {
        foo: String,
    }

    let app = Router::new().route("/", post(|input: Xml<Input>| async { input.0.foo }));

    let server = TestServer::new(app).unwrap();
    let res = server.post("/").text(r#"{ "foo": "bar" }"#).await;

    let status = res.status_code();

    assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

#[tokio::test]
async fn xml_content_types() {
    async fn valid_xml_content_type(content_type: &str) -> bool {
        println!("testing {content_type:?}");

        let app = Router::new().route("/", post(|Xml(_): Xml<Value>| async {}));

        let res = TestServer::new(app)
            .unwrap()
            .post("/")
            .text("<Value/>")
            .content_type(content_type)
            .await;

        res.status_code() == StatusCode::OK
    }

    assert!(valid_xml_content_type("application/xml").await);
    assert!(valid_xml_content_type("application/xml; charset=utf-8").await);
    assert!(valid_xml_content_type("application/xml;charset=utf-8").await);
    assert!(valid_xml_content_type("application/cloudevents+xml").await);
    assert!(!valid_xml_content_type("text/xml").await);
}

#[tokio::test]
async fn invalid_xml_syntax() {
    let app = Router::new().route("/", post(|_: Xml<Value>| async {}));

    let server = TestServer::new(app).unwrap();
    let res = server
        .post("/")
        .text("<")
        .content_type("application/xml")
        .await;

    assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[derive(Deserialize)]
struct Foo {
    #[allow(dead_code)]
    a: i32,
    #[allow(dead_code)]
    b: Vec<Bar>,
}

#[derive(Deserialize)]
struct Bar {
    #[allow(dead_code)]
    x: i32,
    #[allow(dead_code)]
    y: i32,
}

#[tokio::test]
async fn invalid_xml_data() {
    let app = Router::new().route("/", post(|_: Xml<Foo>| async {}));

    let server = TestServer::new(app).unwrap();
    let res = server
        .post("/")
        .text("<Foo><a>1</a><b><x>2</x></b></Foo>")
        .content_type("application/xml")
        .await;

    assert_eq!(res.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    let body_text = res.text();
    assert_eq!(
        body_text,
        "Failed to deserialize the XML body into the target type: b[0]: missing field `y`"
    );
}

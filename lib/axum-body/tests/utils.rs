#![allow(dead_code)]

use axum_core::body::Body;
use axum_core::extract::Request;
use http::Method;
use http::header;

pub(crate) fn set_form(builder: http::request::Builder, data: impl serde::Serialize) -> Request {
    let bytes = serde_urlencoded::to_string(data)
        .expect("Failed to serialize test data as a urlencoded form");

    builder
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(bytes))
        .unwrap()
}

pub(crate) fn set_json(builder: http::request::Builder, data: impl serde::Serialize) -> Request {
    let bytes = serde_json::to_string(&data).expect("Failed to serialize test data to json");

    builder
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(bytes))
        .unwrap()
}

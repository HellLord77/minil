#![allow(dead_code)]

use axum_core::body::Body;
use axum_core::extract::Request;
use http::Method;
use http::header;
use mime::APPLICATION_JSON;
use mime::APPLICATION_WWW_FORM_URLENCODED;

pub(crate) fn set_form(builder: http::request::Builder, data: impl serde::Serialize) -> Request {
    let bytes = serde_urlencoded::to_string(data)
        .expect("Failed to serialize test data as a urlencoded form");

    builder
        .method(Method::POST)
        .header(
            header::CONTENT_TYPE,
            APPLICATION_WWW_FORM_URLENCODED.as_ref(),
        )
        .body(Body::from(bytes))
        .unwrap()
}

pub(crate) fn set_json(builder: http::request::Builder, data: impl serde::Serialize) -> Request {
    let bytes = serde_json::to_string(&data).expect("Failed to serialize test data to json");

    builder
        .header(header::CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .body(Body::from(bytes))
        .unwrap()
}

#![cfg(feature = "caseless")]

use serde::Deserialize;
use serde_header::de::from_header_seq;
use serde_header::types::HeaderRef;

#[test]
fn deserialize_rename() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct Foo {
        #[serde(rename = "Bar-Baz")]
        bar_baz: String,
    }

    let input: Vec<HeaderRef> = vec![("bar-baz", b"quux")];
    let result = Foo {
        bar_baz: "quux".to_owned(),
    };

    assert_eq!(from_header_seq(&input), Ok(result));
}

#[test]
fn deserialize_rename_all() {
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "UPPERCASE")]
    struct Foo {
        bar: String,
        baz: i32,
    }

    let input: Vec<HeaderRef> = vec![("bar", b"quux"), ("baz", b"42")];
    let result = Foo {
        bar: "quux".to_owned(),
        baz: 42,
    };

    assert_eq!(from_header_seq(&input), Ok(result));
}

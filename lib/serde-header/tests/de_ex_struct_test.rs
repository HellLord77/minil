use serde::Deserialize;
use serde_header::de::from_headers;
use serde_header::types::HeaderRef;

#[test]
fn deserialize_struct() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct Foo {
        bar: String,
        baz: i32,
    }

    let input: Vec<HeaderRef> = vec![("bar", b"quux"), ("baz", b"42")];
    let result = Foo {
        bar: "quux".to_owned(),
        baz: 42,
    };

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_renamed_struct() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct Foo {
        bar: String,
        #[serde(rename = "qux")]
        baz: i32,
    }

    let input: Vec<HeaderRef> = vec![("bar", b"quux"), ("qux", b"42")];
    let result = Foo {
        bar: "quux".to_owned(),
        baz: 42,
    };

    assert_eq!(from_headers(&input), Ok(result));
}

#![cfg(feature = "httparse")]

use serde_header::de::from_bytes;
use serde_header::de::from_reader;
use serde_header::de::from_str;

#[test]
fn deserialize_bytes() {
    let input = b"first: 23\nlast: 42\n\n";
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(from_bytes(input), Ok(result));
}

#[test]
fn deserialize_str() {
    let input = "first: 23\nlast: 42\n\n";
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(from_str(input), Ok(result));
}

#[test]
fn deserialize_reader() {
    let input = b"first: 23\nlast: 42\n\n" as &[_];
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(from_reader(input), Ok(result));
}

#[test]
fn deserialize_unit() {
    let input = b"\n\n";
    assert_eq!(from_bytes(input), Ok(()));

    let input = b"\n\n\r\n";
    assert_eq!(from_bytes(input), Ok(()));
}

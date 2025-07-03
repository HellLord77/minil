#![cfg(feature = "httparse")]

use serde_header::de::try_from_bytes;

#[test]
fn deserialize_bytes() {
    let input = b"first: 23\nlast: 42\n\n";
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(try_from_bytes(input), Ok(Ok(result)));
}

#[test]
fn deserialize_unit() {
    let input = b"\n\n";
    assert_eq!(try_from_bytes(input), Ok(Ok(())));

    let input = b"\n\n\r\n";
    assert_eq!(try_from_bytes(input), Ok(Ok(())));
}

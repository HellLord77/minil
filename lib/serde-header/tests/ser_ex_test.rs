use serde_header::ser::to_bytes;
use serde_header::ser::to_string;
use serde_header::ser::to_writer;

#[test]
fn serialize_bytes() {
    let input = &[("first", 23), ("last", 42)];
    let result = b"first: 23\r\nlast: 42\r\n\r\n".to_vec();

    assert_eq!(to_bytes(input), Ok(result));
}

#[test]
fn serialize_string() {
    let input = vec![("first", 23), ("last", 42)];
    let result = "first: 23\r\nlast: 42\r\n\r\n".to_owned();

    assert_eq!(to_string(input), Ok(result));
}

#[test]
fn serialize_writer() {
    let input = &[("first", 23), ("last", 42)];
    let result = b"first: 23\r\nlast: 42\r\n\r\n".to_vec();

    assert_eq!(to_writer(vec![], input), Ok(result));
}

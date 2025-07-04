#![cfg(feature = "httparse")]

use serde::Deserialize;
use serde_header::de::from_bytes;

#[test]
fn deserialize_partial() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        foo: String,
        bar: String,
    }

    let input = b"foo: qux\nbar: quux\n";
    let result = "too many headers";

    assert_eq!(from_bytes::<Form>(input).unwrap_err().to_string(), result);
}

#[test]
fn deserialize_too_many() {
    let input = format!("{}\n\n", "foo: bar\n".repeat(1000));
    let result = vec![("foo", "bar"); 1000];

    assert_eq!(from_bytes(input.as_bytes()), Ok(result));
}

#![cfg(feature = "strfmt")]

use serde::Deserialize;
use serde_rename_chain::serde_rename_chain;

#[serde_rename_chain(str_fmt = "field_{s}")]
#[derive(Deserialize)]
struct Foo {
    fizz: String,
    buzz: String,
}

#[test]
fn test_self() {
    let foo = serde_json::from_str::<Foo>(r#"{"field_fizz": "foo", "field_buzz": "bar"}"#).unwrap();
    assert_eq!(foo.fizz, "foo");
    assert_eq!(foo.buzz, "bar");
}

#[serde_rename_chain(str_fmt = "field_{l}")]
#[derive(Deserialize)]
struct Bar {
    fiz: String,
    buzzz: String,
}

#[test]
fn test_len() {
    let bar = serde_json::from_str::<Bar>(r#"{"field_3": "foo", "field_5": "bar"}"#).unwrap();
    assert_eq!(bar.fiz, "foo");
    assert_eq!(bar.buzzz, "bar");
}

#[serde_rename_chain(str_fmt = "{s}_{l:0>2}")]
#[derive(Deserialize)]
struct FooBar {
    fizz: String,
    buzz: String,
}

#[test]
fn test_strfmt() {
    let foobar = serde_json::from_str::<FooBar>(r#"{"fizz_04": "foo", "buzz_04": "bar"}"#).unwrap();
    assert_eq!(foobar.fizz, "foo");
    assert_eq!(foobar.buzz, "bar");
}

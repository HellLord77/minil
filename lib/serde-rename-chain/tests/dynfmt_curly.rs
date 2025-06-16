#![cfg(feature = "dynfmt_curly")]

use serde::Deserialize;
use serde_rename_chain::serde_rename_chain;

#[serde_rename_chain(dyn_fmt_curly = "field_{}")]
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

#[serde_rename_chain(dyn_fmt_curly = "field_{1}")]
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

#[serde_rename_chain(dyn_fmt_curly = "{}_{}")]
#[derive(Deserialize)]
struct FooBar {
    fiz: String,
    buzzz: String,
}

#[test]
fn test_dynfmt_curly() {
    let foobar = serde_json::from_str::<FooBar>(r#"{"fiz_3": "foo", "buzzz_5": "bar"}"#).unwrap();
    assert_eq!(foobar.fiz, "foo");
    assert_eq!(foobar.buzzz, "bar");
}

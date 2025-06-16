#![cfg(feature = "dynfmt_python")]

use serde::Deserialize;
use serde_rename_chain::serde_rename_chain;

#[serde_rename_chain(dyn_fmt_python = "field_%s")]
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

#[serde_rename_chain(dyn_fmt_python = "%s_%s")]
#[derive(Deserialize)]
struct FooBar {
    fiz: String,
    buzzz: String,
}

#[test]
fn test_dynfmt_python() {
    let foobar = serde_json::from_str::<FooBar>(r#"{"fiz_3": "foo", "buzzz_5": "bar"}"#).unwrap();
    assert_eq!(foobar.fiz, "foo");
    assert_eq!(foobar.buzzz, "bar");
}

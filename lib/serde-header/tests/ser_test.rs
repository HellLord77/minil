use std::collections::BTreeMap;

use serde::Serialize;
use serde_header::ser::to_headers;
use serde_header::types::HeaderOwnedSeq;

fn map(s: &[(&str, &[u8])]) -> HeaderOwnedSeq {
    s.iter().map(|(k, v)| (k.to_string(), v.to_vec())).collect()
}

#[derive(Serialize)]
struct NewType<T>(T);

#[test]
fn serialize_newtype_i32() {
    let input = &[("field".to_owned(), NewType(11))];
    let result = map(&[("field", b"11")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_newtype_u128() {
    let input = &[("field".to_owned(), Some(NewType(u128::MAX)))];
    let result = map(&[("field", u128::MAX.to_string().as_bytes())]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_newtype_i128() {
    let input = &[("field".to_owned(), Some(NewType(i128::MIN)))];
    let result = map(&[("field", i128::MIN.to_string().as_bytes())]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_option_map_int() {
    let input = &[("first", Some(23)), ("middle", None), ("last", Some(42))];
    let result = map(&[("first", b"23"), ("last", b"42")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_option_map_string() {
    let input = &[
        ("first", Some("hello")),
        ("middle", None),
        ("last", Some("world")),
    ];
    let result = map(&[("first", b"hello"), ("last", b"world")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_option_map_bool() {
    let input = &[("one", Some(true)), ("two", Some(false))];
    let result = map(&[("one", b"true"), ("two", b"false")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_map_bool() {
    let input = &[("one", true), ("two", false)];
    let result = map(&[("one", b"true"), ("two", b"false")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_map_duplicate_keys() {
    let input = &[("foo", "a"), ("foo", "b")];
    let result = map(&[("foo", b"a"), ("foo", b"b")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[derive(Serialize)]
enum X {
    A,
    B,
    C,
}

#[test]
fn serialize_unit_enum() {
    let input = &[("one", X::A), ("two", X::B), ("three", X::C)];
    let result = map(&[("one", b"A"), ("two", b"B"), ("three", b"C")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_unit_struct() {
    #[derive(Serialize)]
    struct Unit;

    let input = Unit;
    let result = map(&[]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_unit_type() {
    let input = ();
    let result = map(&[]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_newstruct() {
    #[derive(Serialize)]
    struct NewStruct {
        list: Vec<String>,
    }

    let input = NewStruct {
        list: vec!["hello".into(), "world".into()],
    };
    let result = map(&[("list", b"hello"), ("list", b"world")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[derive(Serialize)]
struct Wrapper<T> {
    item: T,
}

#[test]
fn serialize_vec_bool() {
    let input = Wrapper {
        item: vec![true, false, false],
    };
    let result = map(&[("item", b"true"), ("item", b"false"), ("item", b"false")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_vec_num() {
    let input = Wrapper {
        item: vec![0, 1, 2],
    };
    let result = map(&[("item", b"0"), ("item", b"1"), ("item", b"2")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_vec_str() {
    let input = Wrapper {
        item: vec!["hello", "world", "hello"],
    };
    let result = map(&[("item", b"hello"), ("item", b"world"), ("item", b"hello")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_struct_opt() {
    #[derive(Serialize)]
    struct Struct {
        list: Vec<Option<String>>,
    }

    let input = Struct {
        list: vec![Some("hello".into()), Some("world".into())],
    };
    let result = map(&[("list", b"hello"), ("list", b"world")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_struct_newtype() {
    #[derive(Serialize)]
    struct ListStruct {
        list: Vec<NewType<usize>>,
    }

    let input = ListStruct {
        list: vec![NewType(0), NewType(1)],
    };
    let result = map(&[("list", b"0"), ("list", b"1")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_struct_unit_enum() {
    let input = Wrapper {
        item: vec![X::A, X::B, X::C],
    };
    let result = map(&[("item", b"A"), ("item", b"B"), ("item", b"C")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_list_of_str() {
    let input = &[("list", vec!["hello", "world"])];
    let result = map(&[("list", b"hello"), ("list", b"world")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_multiple_lists() {
    #[derive(Serialize)]
    struct Lists {
        xs: Vec<bool>,
        ys: Vec<u32>,
    }

    let input = Lists {
        xs: vec![true, false],
        ys: vec![3, 2, 1],
    };
    let result = map(&[
        ("xs", b"true"),
        ("xs", b"false"),
        ("ys", b"3"),
        ("ys", b"2"),
        ("ys", b"1"),
    ]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_nested_list() {
    let input = &[("list", vec![vec![0_u8]])];
    let result = "sequence is not supported value";

    assert_eq!(to_headers(input).unwrap_err().to_string(), result);
}

#[test]
fn serialize_list_of_option() {
    let input = &[("list", vec![Some(10), Some(100)])];
    let result = map(&[("list", b"10"), ("list", b"100")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_list_of_newtype() {
    let input = &[("list", vec![NewType("test".to_owned())])];
    let result = map(&[("list", b"test")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_list_of_enum() {
    let input = &[("item", vec![X::A, X::B, X::C])];
    let result = map(&[("item", b"A"), ("item", b"B"), ("item", b"C")]);

    assert_eq!(to_headers(input), Ok(result));
}

#[test]
fn serialize_map() {
    let input = BTreeMap::from_iter([("a", "hello"), ("b", "world")]);
    let result = map(&[("a", b"hello"), ("b", b"world")]);

    assert_eq!(to_headers(input), Ok(result));
}

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_header::de::from_headers;
use serde_header::types::HeaderRef;

#[test]
fn deserialize_newtype_i32() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct NewType<T>(T);

    let input: Vec<HeaderRef> = vec![("field", b"11")];
    let result = vec![("field".to_owned(), NewType(11))];

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_str() {
    let input: Vec<HeaderRef> = vec![("first", b"23"), ("last", b"42")];
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_borrowed_str() {
    let input: Vec<HeaderRef> = vec![("first", b"23"), ("last", b"42")];
    let result = vec![("first", 23), ("last", 42)];

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_option() {
    let input: Vec<HeaderRef> = vec![("first", b"23"), ("last", b"42")];
    let result = vec![
        ("first".to_owned(), Some(23)),
        ("last".to_owned(), Some(42)),
    ];

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_empty_string() {
    let input: Vec<HeaderRef> = vec![("first", b"")];
    let result = vec![("first".to_owned(), "")];

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_map() {
    let input: Vec<HeaderRef> = vec![("first", b"23"), ("second", b"42")];
    let result = BTreeMap::from_iter([("first".to_owned(), 23), ("second".to_owned(), 42)]);

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_map_vec() {
    let input: Vec<HeaderRef> = vec![("first", b"23"), ("second", b"42"), ("first", b"1")];
    let result = BTreeMap::from_iter([
        ("first".to_owned(), vec![23, 1]),
        ("second".to_owned(), vec![42]),
    ]);

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_tuple_list() {
    let input: Vec<HeaderRef> = vec![("foo", b"1"), ("bar", b"2"), ("foo", b"3")];
    let result = vec![
        ("foo".to_owned(), 1),
        ("bar".to_owned(), 2),
        ("foo".to_owned(), 3),
    ];

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_vec_strings() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Vec<String>,
    }

    let input: Vec<HeaderRef> = vec![("value", b""), ("value", b"abc")];
    let result = Form {
        value: vec!["".to_owned(), "abc".to_owned()],
    };

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_option_vec() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<Vec<String>>,
    }

    let input: Vec<HeaderRef> = vec![];
    let result = Form { value: None };

    assert_eq!(from_headers(&input), Ok(result));

    let input: Vec<HeaderRef> = vec![("value", b"abc")];
    let result = Form {
        value: Some(vec!["abc".to_owned()]),
    };

    assert_eq!(from_headers(&input), Ok(result));

    let input: Vec<HeaderRef> = vec![("value", b"abc"), ("value", b"def")];
    let result = Form {
        value: Some(vec!["abc".to_owned(), "def".to_owned()]),
    };

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_option_vec_int() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<Vec<i32>>,
    }

    let input: Vec<HeaderRef> = vec![];
    let result = Form { value: None };

    assert_eq!(from_headers(&input), Ok(result));

    let input: Vec<HeaderRef> = vec![("value", b"0")];
    let result = Form {
        value: Some(vec![0]),
    };

    assert_eq!(from_headers(&input), Ok(result));

    let input: Vec<HeaderRef> = vec![("value", b"3"), ("value", b"-1")];
    let result = Form {
        value: Some(vec![3, -1]),
    };

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_option_no_value() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<f64>,
    }

    let input: Vec<HeaderRef> = vec![];
    let result = Form { value: None };

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_vec_options_no_value() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Vec<Option<f64>>,
    }

    let input: Vec<HeaderRef> = vec![("value", b""), ("value", b""), ("value", b"")];
    let result = Form {
        value: vec![None, None, None],
    };

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_vec_options_some_values() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Vec<Option<f64>>,
    }

    let input: Vec<HeaderRef> = vec![("value", b""), ("value", b"4"), ("value", b"")];
    let result = Form {
        value: vec![None, Some(4.0), None],
    };

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_option_vec_no_value() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<Vec<f64>>,
    }

    let input: Vec<HeaderRef> = vec![("value", b""), ("value", b""), ("value", b"")];
    let result = "cannot parse float from empty string";

    assert_eq!(
        from_headers::<Form>(&input).unwrap_err().to_string(),
        result
    );
}

#[test]
fn deserialize_option_vec_with_values() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<Vec<f64>>,
    }

    let input: Vec<HeaderRef> = vec![("value", b"3"), ("value", b"4"), ("value", b"5")];
    let result = Form {
        value: Some(vec![3.0, 4.0, 5.0]),
    };

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_no_value_err() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: f64,
    }

    let input: Vec<HeaderRef> = vec![("value", b"")];
    let result = "cannot parse float from empty string";

    assert_eq!(
        from_headers::<Form>(&input).unwrap_err().to_string(),
        result
    );
}

#[test]
fn deserialize_unit() {
    let input: Vec<HeaderRef> = vec![];
    assert_eq!(from_headers(&input), Ok(()));

    let input: Vec<HeaderRef> = vec![("first", b"23")];
    assert!(from_headers::<()>(&input).is_err());
}

#[test]
fn deserialize_unit_enum() {
    #[derive(Debug, PartialEq, Deserialize)]
    enum X {
        A,
        B,
        C,
    }

    let input: Vec<HeaderRef> = vec![("one", b"A"), ("two", b"B"), ("three", b"C")];
    let result = vec![
        ("one".to_owned(), X::A),
        ("two".to_owned(), X::B),
        ("three".to_owned(), X::C),
    ];

    assert_eq!(from_headers(&input), Ok(result));
}

#[test]
fn deserialize_unit_type() {
    let input: Vec<HeaderRef> = vec![];
    assert_eq!(from_headers(&input), Ok(()));
}

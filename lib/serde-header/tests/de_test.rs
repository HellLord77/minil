#![cfg(feature = "http")]

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_header::de::from_header_map;

#[macro_use]
mod utils;

#[test]
fn deserialize_newtype_i32() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct NewType<T>(T);

    let input = map! {
        "field" => "11",
    };
    let result = vec![("field".to_owned(), NewType(11))];

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_str() {
    let input = map! {
        "first" => "23",
        "last" => "42",
    };
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_borrowed_str() {
    let input = map! {
        "first" => "23",
        "last" => "42",
    };
    let result = vec![("first", 23), ("last", 42)];

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_option() {
    let input = map! {
        "first" => "23",
        "last" => "42",
    };
    let result = vec![
        ("first".to_owned(), Some(23)),
        ("last".to_owned(), Some(42)),
    ];

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_empty_string() {
    let input = map! {
        "first" => "",
    };
    let result = vec![("first".to_owned(), "")];

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_map() {
    let input = map! {
        "first" => "23",
        "second" => "42",
    };
    let result = BTreeMap::from_iter([("first".to_owned(), 23), ("second".to_owned(), 42)]);

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_map_vec() {
    let input = map! {
        "first" => "23",
        "second" => "42",
        "first" => "1",
    };
    let result = BTreeMap::from_iter([
        ("first".to_owned(), vec![23, 1]),
        ("second".to_owned(), vec![42]),
    ]);

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_tuple_list() {
    let input = map! {
        "foo" => "1",
        "bar" => "2",
        "foo" => "3",
    };
    let result = vec![
        ("foo".to_owned(), 1),
        ("foo".to_owned(), 3),
        ("bar".to_owned(), 2),
    ];

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_vec_strings() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Vec<String>,
    }

    let input = map! {
        "value" => "",
        "value" => "abc",
    };
    let result = Form {
        value: vec!["".to_owned(), "abc".to_owned()],
    };

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_option_vec() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<Vec<String>>,
    }

    let input = map! {};
    let result = Form { value: None };

    assert_eq!(from_header_map(&input), Ok(result));

    let input = map! {
        "value" => "abc",
    };
    let result = Form {
        value: Some(vec!["abc".to_owned()]),
    };

    assert_eq!(from_header_map(&input), Ok(result));

    let input = map! {
        "value" => "abc",
        "value" => "def",
    };
    let result = Form {
        value: Some(vec!["abc".to_owned(), "def".to_owned()]),
    };

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_option_vec_int() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<Vec<i32>>,
    }

    let input = map! {};
    let result = Form { value: None };

    assert_eq!(from_header_map(&input), Ok(result));

    let input = map! {
        "value" => "0",
    };
    let result = Form {
        value: Some(vec![0]),
    };

    assert_eq!(from_header_map(&input), Ok(result));

    let input = map! {
        "value" => "3",
        "value" => "-1",
    };
    let result = Form {
        value: Some(vec![3, -1]),
    };

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_option_no_value() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<f64>,
    }

    let input = map! {
        "value" => "",
    };
    let result = Form { value: None };

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_vec_options_no_value() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Vec<Option<f64>>,
    }

    let input = map! {
        "value" => "",
        "value" => "",
        "value" => "",
    };

    let result = Form {
        value: vec![None, None, None],
    };

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_vec_options_some_values() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Vec<Option<f64>>,
    }

    let input = map! {
        "value" => "",
        "value" => "4",
        "value" => "",
    };
    let result = Form {
        value: vec![None, Some(4.0), None],
    };

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_option_vec_no_value() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<Vec<f64>>,
    }

    let input = map! {
        "value" => "",
        "value" => "",
        "value" => "",
    };
    let result = "cannot parse float from empty string";

    assert_eq!(
        from_header_map::<Form>(&input).unwrap_err().to_string(),
        result
    );
}

#[test]
fn deserialize_option_vec_with_values() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<Vec<f64>>,
    }

    let input = map! {
        "value" => "3",
        "value" => "4",
        "value" => "5",
    };
    let result = Form {
        value: Some(vec![3.0, 4.0, 5.0]),
    };

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_no_value_err() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: f64,
    }

    let input = map! {
        "value" => "",
    };
    let result = "cannot parse float from empty string";

    assert_eq!(
        from_header_map::<Form>(&input).unwrap_err().to_string(),
        result
    );
}

#[test]
fn deserialize_unit() {
    let input = map! {};
    let result = ();

    assert_eq!(from_header_map(&input), Ok(result));

    let input = map! {
        "first" => "23",
    };

    assert!(from_header_map::<()>(&input).is_err());
}

#[test]
fn deserialize_unit_enum() {
    #[derive(Debug, PartialEq, Deserialize)]
    enum X {
        A,
        B,
        C,
    }

    let input = map! {
        "one" => "A",
        "two" => "B",
        "three" => "C",
    };
    let result = vec![
        ("one".to_owned(), X::A),
        ("two".to_owned(), X::B),
        ("three".to_owned(), X::C),
    ];

    assert_eq!(from_header_map(&input), Ok(result));
}

#[test]
fn deserialize_unit_type() {
    let input = map! {};
    let result = ();

    assert_eq!(from_header_map(&input), Ok(result));
}

#![allow(warnings)]
#![allow(clippy::all)]

use std::marker::PhantomData;

use stringify_checked::stringify_expr;
use trybuild::TestCases;

#[test]
fn test_simple() {
    assert_eq!(stringify_expr!(()), "()");
    assert_eq!(stringify_expr!(1 + 2), "1 + 2");
    assert_eq!(stringify_expr!(!true), "!true");
    assert_eq!(stringify_expr!(b""), "b\"\"");
}

#[test]
fn test_compound() {
    assert_eq!(stringify_expr!(false.to_string()), "false.to_string()");
    assert_eq!(stringify_expr!(Some("test")), "Some(\"test\")");
    assert_eq!(stringify_expr!(vec![1, 2, 3]), "vec![1, 2, 3]");
    assert_eq!(stringify_expr!(PhantomData::<i32>), "PhantomData::<i32>");
    assert_eq!(
        stringify_expr!(String::from("test")),
        "String::from(\"test\")"
    );
}

#[test]
fn test_complex() {
    assert_eq!(
        stringify_expr!({
            let _a = 5;
        }),
        "{ let _a = 5; }"
    );
    assert_eq!(
        stringify_expr!(if 'a'.is_alphabetic() { 1 } else { 2 }),
        "if 'a'.is_alphabetic() { 1 } else { 2 }"
    );
    assert_eq!(
        stringify_expr!(match Some(1) {
            Some(x) => 1 + x,
            None => 0,
        }),
        "match Some(1) { Some(x) => 1 + x, None => 0, }"
    );
    assert_eq!(
        stringify_expr!(for i in 0..10 {
            i;
        }),
        "for i in 0..10 { i; }"
    );
    assert_eq!(
        stringify_expr!(loop {
            break;
        }),
        "loop { break; }"
    );
    assert_eq!(
        stringify_expr!({
            fn _foo() -> i32 {
                42
            }
        }),
        "{ fn _foo() -> i32 { 42 } }"
    );
}

#[test]
fn test_compile_fail() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/stringify_expr/*.rs");
}

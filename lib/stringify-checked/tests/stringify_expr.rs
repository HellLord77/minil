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
fn test_complex() {
    assert_eq!(stringify_expr!(false.to_string()), "false.to_string()");
    assert_eq!(stringify_expr!(Some("test")), "Some(\"test\")");
    assert_eq!(stringify_expr!(vec![1, 2, 3]), "vec![1, 2, 3]");
    assert_eq!(stringify_expr!(PhantomData::<i32>), "PhantomData::<i32>");
}

#[test]
fn test_compile_fail() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/stringify_expr/*.rs");
}

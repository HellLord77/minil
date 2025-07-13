use std::marker::PhantomData;

use stringify_extra::stringify_ty;
use trybuild::TestCases;

#[test]
fn test_primitive() {
    assert_eq!(stringify_ty!(()), "()");
    assert_eq!(stringify_ty!(i32), "i32");
    assert_eq!(stringify_ty!(f64), "f64");
    assert_eq!(stringify_ty!(bool), "bool");
}

#[test]
fn test_generic() {
    assert_eq!(stringify_ty!(Option<&str>), "Option<&str>");
    assert_eq!(stringify_ty!(Result<i32, String>), "Result<i32, String>");
    assert_eq!(stringify_ty!(Vec<String>), "Vec<String>");
    assert_eq!(stringify_ty!(PhantomData<i32>), "PhantomData<i32>");
}

#[test]
fn test_compile_fail() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/stringify_ty/*.rs");
}

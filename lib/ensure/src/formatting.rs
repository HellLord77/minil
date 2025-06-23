use core::fmt;
use std::fmt::Debug;
use std::panic::Location;

#[derive(Debug)]
#[doc(hidden)]
pub enum EnsureKind {
    Eq,
    Ne,
    Match,
}

#[track_caller]
#[doc(hidden)]
pub fn ensure_failed<T, U>(kind: EnsureKind, left: &T, right: &U) -> String
where
    T: Debug + ?Sized,
    U: Debug + ?Sized,
{
    ensure_failed_inner(kind, &left, &right)
}

#[track_caller]
#[doc(hidden)]
pub fn ensure_matches_failed<T: Debug + ?Sized>(left: &T, right: &str) -> String {
    struct Pattern<'a>(&'a str);
    impl Debug for Pattern<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.0)
        }
    }
    ensure_failed_inner(EnsureKind::Match, &left, &Pattern(right))
}

#[track_caller]
fn ensure_failed_inner(kind: EnsureKind, left: &dyn Debug, right: &dyn Debug) -> String {
    let caller = Location::caller();
    let op = match kind {
        EnsureKind::Eq => "==",
        EnsureKind::Ne => "!=",
        EnsureKind::Match => "matches",
    };

    format!(
        r#"[{}:{}] assurance `left {op} right` failed
  left: {left:?}
 right: {right:?}"#,
        caller.file(),
        caller.line()
    )
}

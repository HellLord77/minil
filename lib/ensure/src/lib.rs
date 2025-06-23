pub mod formatting;

#[macro_export]
macro_rules! ensure {
    ($cond:expr $(,)?) => {
        if !$cond {
            let dbg = format!(r#"assurance failed: {}"#, ::std::stringify!($cond));
            ::std::eprintln!("{dbg}");
        }
    };
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            let dbg = format!(r#"assurance failed: {}"#, ::std::stringify!($cond));
            ::std::eprintln!("{dbg}");

            return ::std::result::Result::Err($err);
        }
    };
}

#[macro_export]
macro_rules! ensure_eq {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = $crate::formatting::EnsureKind::Eq;
                    let dbg = $crate::formatting::ensure_failed(kind, &*left_val, &*right_val);
                    ::std::eprintln!("{dbg}");
                }
            }
        }
    };
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = $crate::formatting::EnsureKind::Eq;
                    let dbg = $crate::formatting::ensure_failed(kind, &*left_val, &*right_val);
                    ::std::eprintln!("{dbg}");

                    return ::std::result::Result::Err($err);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! ensure_ne {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    let kind = $crate::formatting::EnsureKind::Ne;
                    let dbg = $crate::formatting::ensure_failed(kind, &*left_val, &*right_val);
                    ::std::eprintln!("{dbg}");
                }
            }
        }
    };
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    let kind = $crate::formatting::EnsureKind::Ne;
                    let dbg = $crate::formatting::ensure_failed(kind, &*left_val, &*right_val);
                    ::std::eprintln!("{dbg}");

                    return ::std::result::Result::Err($err);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! ensure_matches {
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {}
            ref left_val => {
                let dbg = $crate::formatting::ensure_matches_failed(
                    left_val,
                    ::std::stringify!($($pattern)|+ $(if $guard)?)
                );
                ::std::eprintln!("{dbg}");
            }
        }
    };
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $err:expr $(,)?) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {}
            ref left_val => {
                let dbg = $crate::formatting::ensure_matches_failed(
                    left_val,
                    ::std::stringify!($($pattern)|+ $(if $guard)?)
                );
                ::std::eprintln!("{dbg}");

                return ::std::result::Result::Err($err);
            }
        }
    }
}

#[macro_export]
macro_rules! debug_ensure {
    ($cond:expr $(,)?) => {
        if ::std::cfg!(debug_assertions) {
            $crate::ensure!($cond);
        }
    };
    ($cond:expr, $err:expr $(,)?) => {
        if ::std::cfg!(debug_assertions) {
            $crate::ensure!($cond, $err);
        }
    };
}

#[macro_export]
macro_rules! debug_ensure_eq {
    ($left:expr, $right:expr $(,)?) => {
        if ::std::cfg!(debug_assertions) {
            $crate::ensure_eq!($left, $right);
        }
    };
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        if ::std::cfg!(debug_assertions) {
            $crate::ensure_eq!($left, $right, $err);
        }
    };
}

#[macro_export]
macro_rules! debug_ensure_ne {
    ($left:expr, $right:expr $(,)?) => {
        if ::std::cfg!(debug_assertions) {
            $crate::ensure_ne!($left, $right);
        }
    };
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        if ::std::cfg!(debug_assertions) {
            $crate::ensure_ne!($left, $right, $err);
        }
    };
}

#[macro_export]
macro_rules! debug_ensure_matches {
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        if ::std::cfg!(debug_assertions) {
            $crate::ensure_matches!($left, $(|)? $( $pattern )|+ $( if $guard )?);
        }
    };
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $err:expr $(,)?) => {
        if ::std::cfg!(debug_assertions) {
            $crate::ensure_matches!($left, $(|)? $( $pattern )|+ $( if $guard )?, $err);
        }
    };
}

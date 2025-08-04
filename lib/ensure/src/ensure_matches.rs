#[macro_export]
macro_rules! is_ensure_matches {
    ($left:expr, $(|)? $($pattern:pat_param)|+ $(if $guard:expr)? $(,)?) => {
        match $left {
            $($pattern)|+ $(if $guard)? => {
                true
            }
            ref left_val => {
                $crate::debug_debug!(
                    r#"assurance `left matches right` failed
  left: {left_val:?}
 right: {}"#,
                    ::core::stringify!($($pattern)|+ $(if $guard)?)
                );

                false
            }
        }
    };
}

#[macro_export]
macro_rules! check_ensure_matches {
    ($left:expr, $(|)? $($pattern:pat_param)|+ $(if $guard:expr)?, $err:expr $(,)?) => {
        !($crate::is_ensure_matches!($left, $($pattern)|+ $(if $guard)?)).then_some($err)
    };
}

#[macro_export]
macro_rules! ensure_matches {
    ($left:expr, $(|)? $($pattern:pat_param)|+ $(if $guard:expr)?, $err:expr $(,)?) => {
        if !$crate::is_ensure_matches!($left, $($pattern)|+ $(if $guard)?) {
            ::core::result::Result::Err($err)?
        }
    };
}

#[macro_export]
macro_rules! panic_ensure_matches {
    ($left:expr, $(|)? $($pattern:pat_param)|+ $(if $guard:expr)?) => {
        $crate::panic_ensure_matches!($left, $($pattern)|+ $(if $guard)?,);
    };
    ($left:expr, $(|)? $($pattern:pat_param)|+ $(if $guard:expr)?, $($arg:tt)*) => {
        if !$crate::is_ensure_matches!($left, $($pattern)|+ $(if $guard)?) {
            ::core::panic!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug_ensure_matches {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::ensure_matches!($($arg:tt)*);
    };
}

#[macro_export]
macro_rules! debug_panic_ensure_matches {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::panic_ensure_matches!($($arg:tt)*);
    };
}

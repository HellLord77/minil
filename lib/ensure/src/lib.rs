// ensure

#[macro_export]
#[doc(hidden)]
macro_rules! __maybe_get_ensure {
    ($cond:expr $(,)?) => {
        $crate::__maybe_get_ensure!($cond, ::std::option::Option::None::<()>)
    };
    ($cond:expr, $err:expr $(,)?) => {
        if $cond {
            ::std::option::Option::None
        } else {
            if cfg!(debug_assertions) {
                let caller = ::std::panic::Location::caller();
                ::std::eprintln!(
                    r#"[{}:{}] assurance failed: {}"#,
                    caller.file(),
                    caller.line(),
                    ::std::stringify!($cond)
                );
            }

            $err
        }
    };
}

#[macro_export]
macro_rules! check_ensure {
    ($cond:expr $(,)?) => {
        $crate::__maybe_get_ensure!($cond)
    };
    ($cond:expr, $err:expr $(,)?) => {
        $crate::__maybe_get_ensure!($cond, ::std::option::Option::Some($err))
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr $(,)?) => {
        $crate::check_ensure!($cond);
    };
    ($cond:expr, $err:expr $(,)?) => {
        if let ::std::option::Option::Some(err) = $crate::check_ensure!($cond, $err) {
            return ::std::result::Result::Err(err);
        }
    };
}

#[macro_export]
macro_rules! debug_ensure {
    ($($arg:tt)*) => {
        if ::std::cfg!(debug_assertions) {
            $crate::ensure!($($arg:tt)*);
        }
    };
}

// ensure_eq

#[macro_export]
#[doc(hidden)]
macro_rules! __maybe_get_ensure_eq {
    ($left:expr, $right:expr $(,)?) => {
        $crate::__maybe_get_ensure_eq!($left, $right, ::std::option::Option::None::<()>)
    };
    ($left:expr, $right:expr, $err:expr $(,)?) => {{
        let left_val = &$left;
        let right_val = &$right;

        if *left_val == *right_val {
            ::std::option::Option::None
        } else {
            if cfg!(debug_assertions) {
                let caller = ::std::panic::Location::caller();
                ::std::eprintln!(
                    r#"[{}:{}] assurance `left == right` failed
  left: {left_val:?}
 right: {right_val:?}"#,
                    caller.file(),
                    caller.line()
                );
            }

            $err
        }
    }};
}

#[macro_export]
macro_rules! check_ensure_eq {
    ($left:expr, $right:expr $(,)?) => {
        $crate::__maybe_get_ensure_eq!($left, $right)
    };
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        $crate::__maybe_get_ensure_eq!($left, $right, ::std::option::Option::Some($err))
    };
}

#[macro_export]
macro_rules! ensure_eq {
    ($left:expr, $right:expr $(,)?) => {
        $crate::check_ensure_eq!($left, $right);
    };
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        if let ::std::option::Option::Some(err) = $crate::check_ensure_eq!($left, $right, $err) {
            return ::std::result::Result::Err(err);
        }
    };
}

#[macro_export]
macro_rules! debug_ensure_eq {
    ($($arg:tt)*) => {
        if ::std::cfg!(debug_assertions) {
            $crate::ensure_eq!($($arg:tt)*);
        }
    };
}

// ensure_ne

#[macro_export]
#[doc(hidden)]
macro_rules! __maybe_get_ensure_ne {
    ($left:expr, $right:expr $(,)?) => {
        $crate::__maybe_get_ensure_ne!($left, $right, ::std::option::Option::None::<()>)
    };
    ($left:expr, $right:expr, $err:expr $(,)?) => {{
        let left_val = &$left;
        let right_val = &$right;

        if *left_val == *right_val {
            if cfg!(debug_assertions) {
                let caller = ::std::panic::Location::caller();
                ::std::eprintln!(
                    r#"[{}:{}] assurance `left != right` failed
  left: {left_val:?}
 right: {right_val:?}"#,
                    caller.file(),
                    caller.line()
                );
            }

            $err
        } else {
            ::std::option::Option::None
        }
    }};
}

#[macro_export]
macro_rules! check_ensure_ne {
    ($left:expr, $right:expr $(,)?) => {
        $crate::__maybe_get_ensure_ne!($left, $right)
    };
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        $crate::__maybe_get_ensure_ne!($left, $right, ::std::option::Option::Some($err))
    };
}

#[macro_export]
macro_rules! ensure_ne {
    ($left:expr, $right:expr $(,)?) => {
        $crate::check_ensure_ne!($left, $right);
    };
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        if let ::std::option::Option::Some(err) = $crate::check_ensure_ne!($left, $right, $err) {
            return ::std::result::Result::Err(err);
        }
    };
}

#[macro_export]
macro_rules! debug_ensure_ne {
    ($($arg:tt)*) => {
        if ::std::cfg!(debug_assertions) {
            $crate::ensure_ne!($($arg:tt)*);
        }
    };
}

// ensure_matches

#[macro_export]
#[doc(hidden)]
macro_rules! __maybe_get_ensure_matches {
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        $crate::__maybe_get_ensure_matches!($left, $(|)? $( $pattern )|+ $( if $guard )?, ::std::option::Option::None::<()>)
    };
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $err:expr $(,)?) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {
                ::std::option::Option::None
            }
            ref left_val => {
                if cfg!(debug_assertions) {
                    let caller = ::std::panic::Location::caller();
                        ::std::eprintln!(
                            r#"[{}:{}] assurance `left matches right` failed
  left: {left_val:?}
 right: {}"#,
                            caller.file(),
                            caller.line(),
                            ::std::stringify!($($pattern)|+ $(if $guard)?)
                        );
                }

                $err
            }
        }
    };
}

#[macro_export]
macro_rules! check_ensure_matches {
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        $crate::__maybe_get_ensure_matches!($left, $( $pattern )|+ $( if $guard )?)
    };
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $err:expr $(,)?) => {
        $crate::__maybe_get_ensure_matches!($left, $( $pattern )|+ $( if $guard )?, ::std::option::Option::Some($err))
    };
}

#[macro_export]
macro_rules! ensure_matches {
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        $crate::check_ensure_matches!($left, $( $pattern )|+ $( if $guard )?);
    };
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $err:expr $(,)?) => {
        if let ::std::option::Option::Some(err) = $crate::check_ensure_matches!($left, $( $pattern )|+ $( if $guard )?, $err) {
            return ::std::result::Result::Err(err);
        }
    };
}

#[macro_export]
macro_rules! debug_ensure_matches {
    ($($arg:tt)*) => {
        if ::std::cfg!(debug_assertions) {
            $crate::ensure_matches!($($arg:tt)*);
        }
    };
}

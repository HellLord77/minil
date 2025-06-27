// eprintln

#[macro_export]
macro_rules! eprintln {
    () => {
        ::std::eprintln!("[{}:{}:{}]", ::std::file!(), ::std::line!(), ::std::column!())
    };
    ($($arg:tt)*) => {
        match ::std::format_args!($($arg)*) {
            tmp => {
                ::std::eprintln!("[{}:{}:{}] {}",
                    ::std::file!(), ::std::line!(), ::std::column!(), tmp);
                tmp
            }
        }
    };
}

#[macro_export]
macro_rules! debug_eprintln {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::eprintln!($($arg)*);
    };
}

// ensure

#[macro_export]
macro_rules! is_ensure {
    ($cond:expr $(,)?) => {
        if $cond {
            true
        } else {
            $crate::debug_eprintln!(r#"assurance failed: {}"#, ::std::stringify!($cond));

            false
        }
    };
}

#[macro_export]
macro_rules! check_ensure {
    ($cond:expr, $err:expr $(,)?) => {
        !($crate::is_ensure!($cond)).then_some($err)
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr $(,)?) => {
        if !$crate::is_ensure!($cond) {
            ::std::result::Result::Err(err)?
        }
    };
}

#[macro_export]
macro_rules! panic_ensure {
    ($cond:expr) => {
        $crate::panic_ensure!($cond,);
    };
    ($cond:expr, $($arg:tt)*) => {
        if !$crate::is_ensure!($cond) {
            ::std::panic!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug_ensure {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::ensure!($($arg:tt)*);
    };
}

#[macro_export]
macro_rules! debug_panic_ensure {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::panic_ensure!($($arg:tt)*);
    };
}

// ensure_eq

#[macro_export]
macro_rules! is_ensure_eq {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    true
                } else {
                    $crate::debug_eprintln!(
                        r#"assurance `left == right` failed
  left: {left_val:?}
 right: {right_val:?}"#
                    );

                    false
                }
            }
        }
    };
}

#[macro_export]
macro_rules! check_ensure_eq {
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        !($crate::is_ensure_eq!($left, $right)).then_some($err)
    };
}

#[macro_export]
macro_rules! ensure_eq {
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        if !$crate::is_ensure_eq!($left, $right) {
            ::std::result::Result::Err($err)?
        }
    };
}

#[macro_export]
macro_rules! panic_ensure_eq {
    ($left:expr, $right:expr) => {
        $crate::panic_ensure_eq!($left, $right,);
    };
    ($left:expr, $right:expr, $($arg:tt)*) => {
        if !$crate::is_ensure_eq!($left, $right) {
            ::std::panic!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug_ensure_eq {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::ensure_eq!($($arg:tt)*);
    };
}

#[macro_export]
macro_rules! debug_panic_ensure_eq {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::panic_ensure_eq!($($arg:tt)*);
    };
}

// ensure_ne

#[macro_export]
macro_rules! is_ensure_ne {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    $crate::debug_eprintln!(
                        r#"assurance `left != right` failed
  left: {left_val:?}
 right: {right_val:?}"#
                    );

                    false
                } else {
                    true
                }
            }
        }
    };
}

#[macro_export]
macro_rules! check_ensure_ne {
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        !($crate::is_ensure_ne!($left, $right)).then_some($err)
    };
}

#[macro_export]
macro_rules! ensure_ne {
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        if !$crate::is_ensure_ne!($left, $right) {
            ::std::result::Result::Err($err)?
        }
    };
}

#[macro_export]
macro_rules! panic_ensure_ne {
    ($left:expr, $right:expr) => {
        $crate::panic_ensure_ne!($left, $right,);
    };
    ($left:expr, $right:expr, $($arg:tt)*) => {
        if !$crate::is_ensure_ne!($left, $right) {
            ::std::panic!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug_ensure_ne {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::ensure_ne!($($arg:tt)*);
    };
}

#[macro_export]
macro_rules! debug_panic_ensure_ne {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::panic_ensure_ne!($($arg:tt)*);
    };
}

// ensure_matches

#[macro_export]
macro_rules! is_ensure_matches {
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {
                true
            }
            ref left_val => {
                $crate::debug_eprintln!(
                    r#"assurance `left matches right` failed
  left: {left_val:?}
 right: {}"#,
                    ::std::stringify!($($pattern)|+ $(if $guard)?)
                );

                false
            }
        }
    };
}

#[macro_export]
macro_rules! check_ensure_matches {
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $err:expr $(,)?) => {
        !($crate::is_ensure_matches!($left, $($pattern)|+ $(if $guard)?)).then_some($err)
    };
}

#[macro_export]
macro_rules! ensure_matches {
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $err:expr $(,)?) => {
        if !$crate::is_ensure_matches!($left, $($pattern)|+ $(if $guard)?) {
            ::std::result::Result::Err($err)?
        }
    };
}

#[macro_export]
macro_rules! panic_ensure_matches {
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?) => {
        $crate::panic_ensure_matches!($left, $($pattern)|+ $(if $guard)?,);
    };
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $($arg:tt)*) => {
        if !$crate::is_ensure_matches!($left, $($pattern)|+ $(if $guard)?) {
            ::std::panic!($($arg)*);
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

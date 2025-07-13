#[macro_export]
macro_rules! is_ensure_eq {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    true
                } else {
                    $crate::debug_debug!(
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
            ::core::result::Result::Err($err)?
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
            ::core::panic!($($arg)*);
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

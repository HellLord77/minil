#[macro_export]
macro_rules! is_ensure {
    ($cond:expr $(,)?) => {
        if $cond {
            true
        } else {
            $crate::debug_debug!(r#"assurance failed: {}"#, ::core::stringify!($cond));

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
            ::core::result::Result::Err(err)?
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
            ::core::panic!($($arg)*);
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

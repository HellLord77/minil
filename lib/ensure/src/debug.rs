#[macro_export]
macro_rules! debug {
    () => {
        ::std::eprintln!("[{}:{}:{}]", ::core::file!(), ::core::line!(), ::core::column!())
    };
    ($($arg:tt)*) => {
        match ::std::format!($($arg)*) {
            tmp => {
                ::std::eprintln!("[{}:{}:{}] {}",
                    ::core::file!(), ::core::line!(), ::core::column!(), tmp);
                tmp
            }
        }
    };
}

#[macro_export]
macro_rules! debug_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::debug!($($arg)*);
    };
}

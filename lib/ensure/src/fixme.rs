#[macro_export]
macro_rules! fixme {
    () => {
        $crate::debug!("not yet implemented")
    };
    ($($arg:tt)+) => {
        match ::std::format!($($arg)*) {
            tmp => {
                $crate::debug!("not yet implemented: {tmp}");
                tmp
            }
        }
    };
}

#[macro_export]
macro_rules! debug_fixme {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::fixme!($($arg)*);
    };
}

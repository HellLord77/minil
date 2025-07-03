#![cfg(feature = "http")]

#[allow(unused_macros)]
macro_rules! map {
    ($($name:literal => $value:literal),* $(,)?) => {
        ::http::HeaderMap::from_iter(
            ::std::vec![
                $(
                    (
                        ::http::HeaderName::from_static($name),
                        ::http::HeaderValue::from_static($value),
                    ),
                )*
            ]
        )
    }
}

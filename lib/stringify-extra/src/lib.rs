#[macro_export]
macro_rules! stringify_ty {
    ($t:ty) => {{
        let _ = ::core::marker::PhantomData::<$t>;
        ::core::stringify!($t)
    }};
}

#[macro_export]
macro_rules! stringify_expr {
    ($e:expr) => {{
        let _ = &$e;
        ::core::stringify!($e)
    }};
}

mod attr;
mod attrs;
mod ty;

use std::env;

use quote::ToTokens;
use quote::quote;
use syn::__private::TokenStream;
use syn::parse;
use syn::parse::Parse;

pub use crate::attr::Combine;
pub use crate::attr::combine_attribute;
pub use crate::attr::parse_assignment_attribute;
pub use crate::attr::parse_attrs;
pub use crate::attr::parse_parenthesized_attribute;
pub use crate::attrs::has_attribute;
pub use crate::ty::peel_option;
pub use crate::ty::peel_result_ok;

#[macro_export]
macro_rules! bail {
    ($span:expr, $message:literal $(,)?) => {
        return ::std::result::Result::Err(syn::Error::new($span, $message));
    };
    ($span:expr, $message:expr $(,)?) => {
        return ::std::result::Result::Err(::syn::Error::new($span, $message));
    };
    ($span:expr, $fmt:expr, $($arg:tt)*) => {
        return ::std::result::Result::Err(::syn::Error::new($span, ::std::format!($fmt, $($arg)*)));
    };
}

#[macro_export]
macro_rules! bail_spanned {
    ($tokens:expr, $message:literal $(,)?) => {
        return ::std::result::Result::Err(syn::Error::new_spanned($tokens, $message));
    };
    ($tokens:expr, $message:expr $(,)?) => {
        return ::std::result::Result::Err(::syn::Error::new_spanned($tokens, $message));
    };
    ($tokens:expr, $fmt:expr, $($arg:tt)*) => {
        return ::std::result::Result::Err(::syn::Error::new_spanned($tokens, ::std::format!($fmt, $($arg)*)));
    };
}

pub fn expand_with<F, I, K>(input: TokenStream, f: F) -> TokenStream
where
    F: FnOnce(I) -> syn::Result<K>,
    I: Parse,
    K: ToTokens,
{
    expand(parse(input).and_then(f))
}

pub fn expand_attr_with<F, A, I, K>(attr: TokenStream, input: TokenStream, f: F) -> TokenStream
where
    F: FnOnce(A, I) -> K,
    A: Parse,
    I: Parse,
    K: ToTokens,
{
    let expand_result = (|| {
        let attr = parse(attr)?;
        let input = parse(input)?;
        Ok(f(attr, input))
    })();
    expand(expand_result)
}

pub fn expand<T>(res: syn::Result<T>) -> TokenStream
where
    T: ToTokens,
{
    match res {
        Ok(tokens) => {
            let tokens = quote! { #tokens }.into();
            if env::var_os("SYN_UTILS_DEBUG").is_some() {
                eprintln!("{tokens}");
            }
            tokens
        }
        Err(err) => err.into_compile_error().into(),
    }
}

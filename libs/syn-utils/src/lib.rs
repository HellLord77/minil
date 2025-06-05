mod attr;
mod fld;
mod ty;

pub use attr::Combine;
pub use attr::combine_attribute;
pub use attr::parse_assignment_attribute;
pub use attr::parse_attrs;
pub use attr::parse_parenthesized_attribute;

pub use fld::field_has_attribute;

pub use ty::peel_option;
pub use ty::peel_result_ok;

use quote::ToTokens;
use quote::quote;
use syn::__private::TokenStream;
use syn::parse;
use syn::parse::Parse;

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

pub fn expand<T>(result: syn::Result<T>) -> TokenStream
where
    T: ToTokens,
{
    match result {
        Ok(tokens) => {
            let tokens = quote! { #tokens }.into();
            if std::env::var_os("SYN_UTILS_DEBUG").is_some() {
                eprintln!("{tokens}");
            }
            tokens
        }
        Err(err) => err.into_compile_error().into(),
    }
}

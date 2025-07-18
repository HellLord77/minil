mod check;
mod check_ass_fn;
mod check_fn;
mod extra;
mod inline_function;

use proc_macro::TokenStream;
use syn_utils::expand_with;

#[proc_macro_attribute]
pub fn validate_inline_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_with(item, inline_function::expand)
}

#[proc_macro_attribute]
pub fn validate_check(_attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_with(item, check::expand)
}

#[proc_macro_attribute]
pub fn validate_check_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_with(item, check_fn::expand)
}

#[proc_macro_attribute]
pub fn validate_check_ass_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_with(item, check_ass_fn::expand)
}

#[proc_macro_attribute]
pub fn validate_extra(_attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_with(item, extra::expand)
}

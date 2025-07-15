mod inline_function;

use proc_macro::TokenStream;
use syn_utils::expand_with;

#[proc_macro_attribute]
pub fn validate_inline_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_with(item, inline_function::expand)
}

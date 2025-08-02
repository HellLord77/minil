mod error_from_common;

use proc_macro::TokenStream;
use syn_utils::expand_with;

#[proc_macro_derive(ErrorFromCommon)]
pub fn derive_error_from_common(input: TokenStream) -> TokenStream {
    expand_with(input, error_from_common::expand)
}

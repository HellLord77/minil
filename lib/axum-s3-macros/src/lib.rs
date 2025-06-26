mod error_from_request_parts;

use proc_macro::TokenStream;
use syn_utils::expand_with;

#[proc_macro_derive(ErrorFromRequestParts)]
pub fn derive_error_from_request_parts(input: TokenStream) -> TokenStream {
    expand_with(input, error_from_request_parts::expand)
}

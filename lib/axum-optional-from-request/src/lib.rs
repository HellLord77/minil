use proc_macro::TokenStream;
use syn_utils::expand_with;

use crate::optional_from_request::tr::Trait;

mod optional_from_request;

#[proc_macro_derive(OptionalFromRequest)]
pub fn derive_optional_from_request(input: TokenStream) -> TokenStream {
    expand_with(input, |item| {
        optional_from_request::expand(item, Trait::OptionalFromRequest)
    })
}

#[proc_macro_derive(OptionalFromRequestParts)]
pub fn derive_optional_from_request_parts(input: TokenStream) -> TokenStream {
    expand_with(input, |item| {
        optional_from_request::expand(item, Trait::OptionalFromRequestParts)
    })
}

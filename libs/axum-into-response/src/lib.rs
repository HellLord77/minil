mod attr;
mod into_response;

use crate::into_response::tr::Trait;
use proc_macro::TokenStream;
use syn_utils::expand_with;

#[proc_macro_derive(IntoResponse, attributes(into_response))]
pub fn derive_into_response(input: TokenStream) -> TokenStream {
    expand_with(input, |item| {
        into_response::expand(item, Trait::IntoResponse)
    })
}

#[proc_macro_derive(IntoResponseParts, attributes(into_response))]
pub fn derive_into_response_parts(input: TokenStream) -> TokenStream {
    expand_with(input, |item| {
        into_response::expand(item, Trait::IntoResponseParts)
    })
}

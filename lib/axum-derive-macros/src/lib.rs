mod attr;
mod from_request;
mod from_request_optional;
mod into_response;
mod optional_from_request;

use proc_macro::TokenStream;
use syn_utils::expand_with;

#[proc_macro_derive(FromRequest, attributes(from_request))]
pub fn derive_from_request(input: TokenStream) -> TokenStream {
    expand_with(input, |item| from_request::expand(item, false))
}

#[proc_macro_derive(FromRequestParts, attributes(from_request))]
pub fn derive_from_request_parts(input: TokenStream) -> TokenStream {
    expand_with(input, |item| from_request::expand(item, true))
}

#[proc_macro_derive(OptionalFromRequest)]
pub fn derive_optional_from_request(input: TokenStream) -> TokenStream {
    expand_with(input, |item| optional_from_request::expand(item, false))
}

#[proc_macro_derive(OptionalFromRequestParts)]
pub fn derive_optional_from_request_parts(input: TokenStream) -> TokenStream {
    expand_with(input, |item| optional_from_request::expand(item, true))
}

#[proc_macro_attribute]
pub fn from_request_optional(_attr: TokenStream, input: TokenStream) -> TokenStream {
    expand_with(input, |item| from_request_optional::modify(item, false))
}

#[proc_macro_attribute]
pub fn from_request_parts_optional(_attr: TokenStream, input: TokenStream) -> TokenStream {
    expand_with(input, |item| from_request_optional::modify(item, true))
}

#[proc_macro_derive(_FromRequestOptional, attributes(from_request_optional))]
pub fn _derive_from_request_optional(input: TokenStream) -> TokenStream {
    expand_with(input, |item| from_request_optional::expand(item, false))
}

#[proc_macro_derive(_FromRequestPartsOptional, attributes(from_request_optional))]
pub fn _derive_from_request_parts_optional(input: TokenStream) -> TokenStream {
    expand_with(input, |item| from_request_optional::expand(item, true))
}

#[proc_macro_derive(IntoResponse, attributes(into_response))]
pub fn derive_into_response(input: TokenStream) -> TokenStream {
    expand_with(input, |item| into_response::expand(item, false))
}

#[proc_macro_derive(IntoResponseParts, attributes(into_response))]
pub fn derive_into_response_parts(input: TokenStream) -> TokenStream {
    expand_with(input, |item| into_response::expand(item, true))
}

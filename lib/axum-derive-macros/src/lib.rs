mod into_response;
mod optional_from_request_via_from_request;

use proc_macro::TokenStream;
use syn_utils::expand_with;

#[proc_macro_derive(IntoResponse, attributes(into_response))]
pub fn derive_into_response(input: TokenStream) -> TokenStream {
    expand_with(input, |item| {
        into_response::expand(item, into_response::Trait::IntoResponse)
    })
}

#[proc_macro_derive(IntoResponseParts, attributes(into_response))]
pub fn derive_into_response_parts(input: TokenStream) -> TokenStream {
    expand_with(input, |item| {
        into_response::expand(item, into_response::Trait::IntoResponseParts)
    })
}

#[proc_macro_derive(OptionalFromRequestViaFromRequest)]
pub fn derive_optional_from_request_via_from_request(input: TokenStream) -> TokenStream {
    expand_with(input, |item| {
        optional_from_request_via_from_request::expand(
            item,
            optional_from_request_via_from_request::Trait::OptionalFromRequest,
        )
    })
}

#[proc_macro_derive(OptionalFromRequestPartsViaFromRequestParts)]
pub fn derive_optional_from_request_parts_via_from_request_parts(
    input: TokenStream,
) -> TokenStream {
    expand_with(input, |item| {
        optional_from_request_via_from_request::expand(
            item,
            optional_from_request_via_from_request::Trait::OptionalFromRequestParts,
        )
    })
}

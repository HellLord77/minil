use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::router_with_state::RouterWithState;

mod arg;
mod filter;
mod filter_router;
mod router;
mod router_with_state;

#[proc_macro]
pub fn axum_filter_handler(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as RouterWithState);

    match filter_router::expand(item) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[cfg(all(feature = "heck", feature = "inflector"))]
compile_error!("The `heck` and `inflector` features are mutually exclusive");

mod rename_chain;
mod renamer;

use crate::rename_chain::rename_chain_impl;
use proc_macro::TokenStream;
use syn::{Item, Meta, Token, parse_macro_input, punctuated::Punctuated};

#[proc_macro_attribute]
pub fn serde_rename_chain(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);
    let item = parse_macro_input!(input as Item);

    rename_chain_impl(args, item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

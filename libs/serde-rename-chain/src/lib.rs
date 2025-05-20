#[cfg(all(feature = "heck", feature = "inflector"))]
compile_error!("the `heck` and `inflector` features are mutually exclusive");

mod error;
mod rename_chain;
mod renamer;
mod str;

#[cfg(feature = "convert_case")]
mod convert_case;
#[cfg(feature = "heck")]
mod heck;
#[cfg(feature = "ident_case")]
mod ident_case;
#[cfg(feature = "inflector")]
mod inflector;

use crate::rename_chain::rename_all_chain_impl;
use proc_macro::TokenStream;
use syn::{Meta, Token, parse_macro_input, punctuated::Punctuated};

#[proc_macro_attribute]
pub fn serde_rename_chain(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    rename_all_chain_impl(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

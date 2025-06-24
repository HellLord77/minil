#[cfg(all(feature = "heck", feature = "inflector"))]
compile_error!("expected at most one of heck and inflector");

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

use proc_macro::TokenStream;
use syn::Meta;
use syn::Token;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn_utils::expand_with;

#[proc_macro_attribute]
pub fn serde_rename_chain(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);
    expand_with(input, |item| rename_chain::expand(args, item))
}

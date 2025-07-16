#[cfg(all(feature = "heck", feature = "inflector"))]
compile_error!("expected at most one of heck or inflector");

mod attr;
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
use syn::parse_macro_input;
use syn_utils::expand_with;

use crate::attr::SerdeRenameChainAttrs;

#[proc_macro_attribute]
pub fn serde_rename_chain(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as SerdeRenameChainAttrs);

    expand_with(input, |item| rename_chain::expand(args, item))
}

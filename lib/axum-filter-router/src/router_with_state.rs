use syn::ExprPath;
use syn::Token;
use syn::braced;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;

use crate::router::Router;

#[derive(Debug)]
pub(super) struct RouterWithState {
    pub(super) state: ExprPath,
    pub(super) routers: Punctuated<Router, Token![,]>,
}

impl Parse for RouterWithState {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let state = input.parse()?;

        let content;
        braced!(content in input);
        let routers = Punctuated::parse_terminated(&content)?;

        Ok(Self { state, routers })
    }
}

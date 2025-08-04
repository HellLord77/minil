use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use syn::Expr;
use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

use crate::filter::Filter;

#[derive(Debug)]
pub(super) struct Router {
    pub(super) filter: Filter,
    pub(super) handler: Expr,
}

impl Parse for Router {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let filter = input.parse::<Filter>()?;
        let _ = input.parse::<Token![=>]>()?;
        let handler = input.parse::<Expr>()?;

        Ok(Self { filter, handler })
    }
}

impl ToTokens for Router {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let filter = &self.filter;
        let handler = &self.handler;
        let guard = (!matches!(filter, Filter::Default)).then(|| quote!(if #filter));

        let stream = quote! {
            #guard {
                return ::axum::handler::Handler::call(#handler, request, state).await;
            }
        };
        tokens.extend(stream);
    }
}

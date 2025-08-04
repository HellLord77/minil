use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use syn::LitStr;
use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

#[derive(Debug)]
pub(super) enum Arg {
    Equals(String),
    StartsWith(String),
    EndsWith(String),
    Contains(String),
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let has_leading_dots = input.parse::<Option<Token![..]>>()?.is_some();
        let literal = input.parse::<LitStr>()?;
        let value = literal.value();
        let has_trailing_dots = input.parse::<Option<Token![..]>>()?.is_some();

        Ok(match (has_leading_dots, has_trailing_dots) {
            (true, true) => Self::Contains(value),
            (true, false) => Self::EndsWith(value),
            (false, true) => Self::StartsWith(value),
            (false, false) => Self::Equals(value),
        })
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let arg = match self {
            Self::Equals(value) => quote!(.eq(#value)),
            Self::StartsWith(value) => quote!(.starts_with(#value)),
            Self::EndsWith(value) => quote!(.ends_with(#value)),
            Self::Contains(value) => quote!(.contains(#value)),
        };

        arg.to_tokens(tokens);
    }
}

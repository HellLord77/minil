use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use syn::Ident;
use syn::Token;
use syn::parenthesized;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn_utils::bail;
use syn_utils::bail_spanned;

use crate::arg::Arg;

#[derive(Debug)]
pub(super) enum Filter {
    Default,

    Method(Arg),
    Scheme(Arg),
    Host(Arg),
    Port(Arg),
    Path(Arg),
    Version(Arg),
    Query(Arg, Option<Arg>),

    Authority(Arg),
    RawQuery(Arg),
    Uri(Arg),

    Header(Arg, Option<Arg>),
    SchemeHeader(Arg),
    HostHeader(Arg),
    Cookie(Arg, Option<Arg>),
}

impl Parse for Filter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(if input.parse::<Option<Token![_]>>()?.is_some() {
            Self::Default
        } else {
            let filter = input.parse::<Ident>()?;

            let content;
            parenthesized!(content in input);
            let mut args = Punctuated::<Arg, Token![,]>::parse_terminated(&content)?.into_iter();
            let Some(arg) = args.next() else {
                bail_spanned!(filter, "unexpected end of args")
            };

            match filter.to_string().as_str() {
                "method" => Self::Method(arg),
                "scheme" => Self::Scheme(arg),
                "host" => Self::Host(arg),
                "port" => Self::Port(arg),
                "path" => Self::Path(arg),
                "version" => Self::Version(arg),
                "query" => Self::Query(arg, pop1(&mut args)?),
                "authority" => Self::Authority(arg),
                "raw_query" => Self::RawQuery(arg),
                "uri" => Self::Uri(arg),
                "header" => Self::Header(arg, pop1(&mut args)?),
                "scheme_header" => Self::SchemeHeader(arg),
                "host_header" => Self::HostHeader(arg),
                "cookie" => Self::Cookie(arg, pop1(&mut args)?),
                _ => bail_spanned!(filter, "unexpected filter"),
            }
        })
    }
}

impl ToTokens for Filter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let stream = match self {
            Self::Default => unreachable!(),
            Self::Method(arg) => quote!(method #arg),
            Self::Scheme(arg) => quote!(scheme #arg),
            Self::Host(arg) => quote!(host #arg),
            Self::Port(arg) => quote!(port #arg),
            Self::Path(arg) => quote!(path #arg),
            Self::Version(arg) => quote!(version #arg),
            Self::Query(arg, None) => match arg {
                Arg::Equals(value) => quote!(query.contains_key(#value)),
                _ => unimplemented!(),
            },
            Self::Query(arg, Some(extra)) => match arg {
                Arg::Equals(value) => quote! {
                    query.get(#value)
                        .map(|values| values.iter().any(|value| value #extra))
                        .unwrap_or_default()
                },
                _ => unimplemented!(),
            },
            Self::Authority(arg) => quote!(authority #arg),
            Self::RawQuery(arg) => quote!(raw_query #arg),
            Self::Uri(arg) => quote!(uri #arg),
            Self::Header(arg, None) => match arg {
                Arg::Equals(value) => quote!(header.contains_key(#value)),
                _ => unimplemented!(),
            },
            Self::Header(arg, Some(extra)) => match arg {
                Arg::Equals(value) => quote! {
                    quote!(header.get_all(#value).iter().any(|value| value #extra))
                },
                _ => unimplemented!(),
            },
            Self::SchemeHeader(arg) => quote!(scheme_header #arg),
            Self::HostHeader(arg) => quote!(host_header #arg),
            Self::Cookie(arg, None) => match arg {
                Arg::Equals(value) => quote!(cookie.get(#value).is_some()),
                _ => unimplemented!(),
            },
            Self::Cookie(arg, Some(extra)) => match arg {
                Arg::Equals(value) => {
                    quote! {
                        cookie.get(#value).map(|value| value.to_string() #extra).unwrap_or_default()
                    }
                }
                _ => unimplemented!(),
            },
        };

        stream.to_tokens(tokens);
    }
}

fn pop1(iter: &mut impl Iterator<Item = Arg>) -> syn::Result<Option<Arg>> {
    let arg = iter.next();

    if iter.next().is_some() {
        bail!(Span::call_site(), "unexpected arg")
    }

    Ok(arg)
}

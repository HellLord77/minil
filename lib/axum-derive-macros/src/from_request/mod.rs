mod attr;
mod tr;

use attr::FromRequestAttrs;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::Item;
use syn::ItemStruct;
use syn::spanned::Spanned;
use syn_utils::bail_spanned;
use syn_utils::parse_attrs;
pub(super) use tr::Trait;

pub(super) fn expand(item: Item, tr: Trait) -> syn::Result<TokenStream> {
    match item {
        Item::Struct(item) => {
            let ItemStruct { attrs, ident, .. } = item;
            let FromRequestAttrs { via } = parse_attrs("from_request", &attrs)?;
            let path = via
                .ok_or_else(|| syn::Error::new(Span::call_site(), "missing `from_request`"))?
                .1;
            let span = path.span();

            Ok(match tr {
                Trait::FromRequest => {
                    quote_spanned! {span=>
                        #[automatically_derived]
                        impl<S> ::axum::extract::FromRequest<S> for #ident
                        where
                            S: ::std::marker::Send + ::std::marker::Sync,
                        {
                            type Rejection = ::axum::response::Response;

                            async fn from_request(
                                req: ::axum::extract::Request,
                                state: &S,
                            ) -> ::std::result::Result<Self, Self::Rejection> {
                                let value = <#path as ::axum::extract::FromRequest<_, _>>::from_request(req, state)
                                .await
                                .map_err(::axum::response::IntoResponse::into_response)?;

                                ::std::result::Result::Ok(<Self as ::std::convert::From<_>>::from(value))
                            }
                        }
                    }
                }
                Trait::FromRequestParts => {
                    quote_spanned! {span=>
                        #[automatically_derived]
                        impl<S> ::axum::extract::FromRequestParts<S> for #ident
                        where
                            S: ::std::marker::Send + ::std::marker::Sync,
                        {
                            type Rejection = ::axum::response::Response;

                            async fn from_request_parts(
                                parts: &mut ::axum::http::request::Parts,
                                state: &S,
                            ) -> ::std::result::Result<Self, Self::Rejection> {
                                let value = <#path as ::axum::extract::FromRequestParts<_>>::from_request_parts(parts, state)
                                .await
                                .map_err(::axum::response::IntoResponse::into_response)?;

                                ::std::result::Result::Ok(<Self as ::std::convert::From<_>>::from(value))
                            }
                        }
                    }
                }
            })
        }
        Item::Enum(_item) => unimplemented!(),
        _ => bail_spanned!(item, "expected struct or enum"),
    }
}

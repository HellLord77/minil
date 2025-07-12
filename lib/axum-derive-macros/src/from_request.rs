use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::Item;
use syn::ItemStruct;
use syn::spanned::Spanned;
use syn_utils::bail_spanned;
use syn_utils::parse_attrs;

use crate::attr::Attrs;

pub(super) fn expand(item: Item, parts: bool) -> syn::Result<TokenStream> {
    match item {
        Item::Struct(item) => {
            let ItemStruct { attrs, ident, .. } = item;
            let Attrs { via } = parse_attrs("from_request", &attrs)?;

            let (_, path) = via.ok_or_else(|| {
                syn::Error::new(Span::call_site(), "missing `#[from_request(via(...))]`")
            })?;
            let span = path.span();

            Ok(if parts {
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
                        ) -> ::core::result::Result<Self, Self::Rejection> {
                            <#path as ::axum::extract::FromRequestParts<_>>::from_request_parts(req, state)
                                .await
                                .map(<Self as ::std::convert::From<_>>::from)
                                .map_err(::axum::response::IntoResponse::into_response)
                        }
                    }
                }
            } else {
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
                        ) -> ::core::result::Result<Self, Self::Rejection> {
                            <#path as ::axum::extract::FromRequest<_, _>>::from_request(req, state)
                                .await
                                .map(<Self as ::std::convert::From<_>>::from)
                                .map_err(::axum::response::IntoResponse::into_response)
                        }
                    }
                }
            })
        }
        Item::Enum(_item) => unimplemented!(),
        _ => bail_spanned!(item, "expected struct or enum"),
    }
}

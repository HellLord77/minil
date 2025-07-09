mod tr;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;
use syn::ItemStruct;
use syn_utils::bail_spanned;
pub(super) use tr::Trait;

pub(super) fn expand(item: Item, tr: Trait) -> syn::Result<TokenStream> {
    match item {
        Item::Struct(item) => {
            let ItemStruct { ident, .. } = item;

            Ok(match tr {
                Trait::OptionalFromRequest => {
                    quote! {
                        #[automatically_derived]
                        impl<S> ::axum::extract::OptionalFromRequest<S> for #ident
                        where
                            S: ::std::marker::Send + ::std::marker::Sync,
                        {
                            type Rejection = ::std::convert::Infallible;

                            async fn from_request(
                                req: ::axum::http::Request<::axum::body::Body>,
                                state: &S,
                            ) -> ::std::result::Result<::std::option::Option<Self>, Self::Rejection> {
                                ::std::result::Result::Ok(
                                    <Self as ::axum::extract::FromRequest<_, _>>::from_request(req, state)
                                        .await
                                        .ok(),
                                )
                            }
                        }
                    }
                }
                Trait::OptionalFromRequestParts => quote! {
                    #[automatically_derived]
                    impl<S> ::axum::extract::OptionalFromRequestParts<S> for #ident
                    where
                        S: ::std::marker::Send + ::std::marker::Sync,
                    {
                        type Rejection = ::std::convert::Infallible;

                        async fn from_request_parts(
                            parts: &mut ::axum::http::request::Parts,
                            state: &S,
                        ) -> ::std::result::Result<::std::option::Option<Self>, Self::Rejection> {
                            ::std::result::Result::Ok(
                                <Self as ::axum::extract::FromRequestParts<_>>::from_request_parts(parts, state)
                                    .await
                                    .ok(),
                            )
                        }
                    }
                },
            })
        }
        Item::Enum(_item) => unimplemented!(),
        _ => bail_spanned!(item, "expected struct or enum"),
    }
}

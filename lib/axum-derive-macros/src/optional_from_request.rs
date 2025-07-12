use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;
use syn::ItemStruct;
use syn_utils::bail_spanned;

pub(super) fn expand(item: Item, parts: bool) -> syn::Result<TokenStream> {
    match item {
        Item::Struct(item) => {
            let ItemStruct { ident, .. } = item;

            Ok(if parts {
                quote! {
                    #[automatically_derived]
                    impl<S> ::axum::extract::OptionalFromRequestParts<S> for #ident
                    where
                        S: ::std::marker::Send + ::std::marker::Sync,
                    {
                        type Rejection = ::std::convert::Infallible;

                        async fn from_request_parts(
                            parts: &mut ::axum::http::request::Parts,
                            state: &S,
                        ) -> ::core::result::Result<::core::option::Option<Self>, Self::Rejection> {
                            ::core::result::Result::Ok(
                                <Self as ::axum::extract::FromRequestParts<_>>::from_request_parts(parts, state)
                                    .await
                                    .ok(),
                            )
                        }
                    }
                }
            } else {
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
                        ) -> ::core::result::Result<::core::option::Option<Self>, Self::Rejection> {
                            ::core::result::Result::Ok(
                                <Self as ::axum::extract::FromRequest<_, _>>::from_request(req, state)
                                    .await
                                    .ok(),
                            )
                        }
                    }
                }
            })
        }
        Item::Enum(_item) => unimplemented!(),
        _ => bail_spanned!(item, "expected struct or enum"),
    }
}

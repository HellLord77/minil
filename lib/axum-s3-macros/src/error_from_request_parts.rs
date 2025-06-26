use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;
use syn::ItemStruct;

pub(super) fn expand(item: Item) -> syn::Result<TokenStream> {
    match item {
        Item::Struct(item) => {
            let ItemStruct { ident, fields, .. } = item;

            let err_ty = fields
                .iter()
                .find(|field| field.ident.clone().unwrap() == "body")
                .unwrap()
                .ty
                .clone();

            Ok(quote! {
                #[automatically_derived]
                impl<S> ::axum::extract::FromRequestParts<S> for #ident
                where
                    S: ::std::marker::Send + ::std::marker::Sync,
                {
                    type Rejection = ::std::convert::Infallible;

                    async fn from_request_parts(parts: &mut ::axum::http::request::Parts, _state: &S) -> ::std::result::Result<Self, Self::Rejection> {
                        let resource = parts.uri.path();
                        let maybe_request_id = parts
                            .headers
                            .get("x-amz-request-id")
                            .and_then(|value| value.to_str().ok());

                        Ok(Self::builder()
                            .body(
                                #err_ty::builder()
                                    .resource(resource)
                                    .maybe_request_id(maybe_request_id)
                                    .build(),
                            )
                            .build())
                    }
                }
            })
        }
        _ => {
            unimplemented!()
        }
    }
}

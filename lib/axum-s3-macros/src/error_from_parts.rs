use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub(super) fn expand(item: ItemStruct) -> syn::Result<TokenStream> {
    let ItemStruct { ident, fields, .. } = item;

    let body_ty = fields
        .iter()
        .find(|field| field.ident.clone().unwrap() == "body")
        .unwrap()
        .ty
        .clone();

    Ok(quote! {
        #[automatically_derived]
        impl ::std::convert::From<crate::utils::ErrorParts> for #ident {
            fn from(parts: crate::utils::ErrorParts) -> Self {
                Self::builder()
                    .body(
                        #body_ty::builder()
                            .resource(parts.resource)
                            .maybe_request_id(parts.request_id)
                            .build(),
                    )
                    .build()
            }
        }
    })
}

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
        impl ::std::convert::From<crate::utils::CommonExtInput> for #ident {
            fn from(common: crate::utils::CommonExtInput) -> Self {
                Self::builder()
                    .body(
                        #body_ty::builder()
                            .resource(common.path.path().to_owned())
                            .request_id(common.header.request_id)
                            .build(),
                    )
                    .build()
            }
        }
    })
}

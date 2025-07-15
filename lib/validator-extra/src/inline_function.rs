use std::mem;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Expr;
use syn::Fields;
use syn::ItemStruct;
use syn::parse_quote;
use syn_utils::bail_spanned;
use syn_utils::peel_option;

pub(super) fn expand(mut item: ItemStruct) -> syn::Result<TokenStream> {
    let ItemStruct {
        ref ident,
        ref mut fields,
        ..
    } = item;
    let mut inline_fns = vec![];

    match fields {
        Fields::Named(fields) => {
            for field in fields.named.iter_mut() {
                let field_ident = field.ident.as_ref().unwrap_or_else(|| unreachable!());
                let field_ty = peel_option(&field.ty).unwrap_or(&field.ty);
                let attrs = mem::take(&mut field.attrs);

                for (attr_index, attr) in attrs.into_iter().enumerate() {
                    if !attr.path().is_ident("validate_inline_function") {
                        field.attrs.push(attr);
                        continue;
                    }

                    let doc = quote!(#attr).to_string();
                    field.attrs.push(parse_quote!(#[doc = #doc]));

                    let fn_name_lit =
                        format!("_validate_inline_function_{ident}_{field_ident}_{attr_index}");
                    let fn_name_ident = format_ident!("{fn_name_lit}");
                    field
                        .attrs
                        .push(parse_quote!(#[validate(custom(function = #fn_name_lit))]));

                    let fn_body = attr.parse_args::<Expr>()?;
                    inline_fns.push(quote! {
                        #[doc(hidden)]
                        #[allow(clippy::ptr_arg)]
                        fn #fn_name_ident(
                            #field_ident: &#field_ty
                        ) -> ::core::result::Result<(), ::validator::ValidationError> {
                            #fn_body
                        }
                    });
                }
            }

            Ok(quote! {
                #(#inline_fns)*
                #item
            })
        }
        _ => bail_spanned!(fields, "expected named fields"),
    }
}

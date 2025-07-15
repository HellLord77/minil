use std::mem;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Expr;
use syn::Fields;
use syn::ItemStruct;
use syn::parse_quote;
use syn_utils::bail_spanned;
use syn_utils::peel_option;

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct Args {
    inline_function: Expr,
    code: Option<String>,
    message: Option<String>,
}

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

                    let args = attr.parse_args::<Args>()?;
                    let code = args.code.map(|code| quote!(code = #code,));
                    let message = args.message.map(|message| quote!(message = #message,));

                    let fn_name_lit =
                        format!("_validate_inline_function_{ident}_{field_ident}_{attr_index}");
                    let fn_name_ident = format_ident!("{fn_name_lit}");
                    field.attrs.push(
                        parse_quote!(#[validate(custom(function = #fn_name_lit, #code #message))]),
                    );

                    let fn_body = args.inline_function;
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

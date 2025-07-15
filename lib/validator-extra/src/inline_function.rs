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

                for (attr_index, attr) in field.attrs.iter_mut().enumerate() {
                    if !attr.path().is_ident("validate_inline_function") {
                        continue;
                    }

                    let fn_name_lit =
                        format!("_validate_inline_function_{ident}_{field_ident}_{attr_index}");
                    let fn_name_ident = format_ident!("{fn_name_lit}");
                    let fn_arg_ty = peel_option(&field.ty).unwrap_or(&field.ty);
                    let fn_body = attr.parse_args::<Expr>()?;

                    inline_fns.push(quote! {
                        #[allow(clippy::ptr_arg)]
                        fn #fn_name_ident(#field_ident: &#fn_arg_ty) -> ::core::result::Result<(), ::validator::ValidationError> {
                            #fn_body
                        }
                    });

                    *attr = parse_quote!(#[validate(custom(function = #fn_name_lit))]);
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

use std::mem;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;
use syn::Fields;
use syn::ItemStruct;
use syn::parse_quote;
use syn_utils::bail_spanned;

pub(super) fn expand(mut item: ItemStruct) -> syn::Result<TokenStream> {
    let ItemStruct {
        ref mut attrs,
        ref mut fields,
        ..
    } = item;
    attrs.push(parse_quote!(#[::validator_extra::validate_inline_function]));

    match fields {
        Fields::Named(fields) => {
            for field in fields.named.iter_mut() {
                let field_ident = field.ident.as_ref().unwrap_or_else(|| unreachable!());
                let field_ident_str = field_ident.to_string();
                let attrs = mem::take(&mut field.attrs);

                for attr in attrs.into_iter() {
                    if !attr.path().is_ident("validate_check") {
                        field.attrs.push(attr);
                        continue;
                    }

                    let doc = quote!(#attr).to_string();
                    field.attrs.push(parse_quote!(#[doc = #doc]));

                    let check_body = attr.parse_args::<Expr>()?;
                    field.attrs.push(parse_quote! {
                        #[validate_inline_function({
                            if #check_body {
                                ::core::result::Result::Ok(())
                            } else {
                                ::core::result::Result::Err(
                                    ::validator::ValidationError::new(#field_ident_str)
                                )
                            }
                        })]
                    });
                }
            }

            Ok(quote!(#item))
        }
        _ => bail_spanned!(fields, "expected named fields"),
    }
}

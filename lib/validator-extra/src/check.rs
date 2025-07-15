use std::mem;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;
use syn::Fields;
use syn::ItemStruct;
use syn::parse_quote;
use syn_utils::bail_spanned;

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct Args {
    check: Expr,
    #[darling(default)]
    invert: bool,
    code: Option<String>,
    message: Option<String>,
}

pub(super) fn expand(mut item: ItemStruct) -> syn::Result<TokenStream> {
    let ItemStruct {
        ref mut attrs,
        ref mut fields,
        ..
    } = item;

    match fields {
        Fields::Named(fields) => {
            attrs.push(parse_quote!(#[::validator_extra::validate_inline_function]));

            for field in fields.named.iter_mut() {
                let field_ident = field.ident.as_ref().unwrap_or_else(|| unreachable!());
                let field_ident_str = field_ident.to_string();
                let attrs = mem::take(&mut field.attrs);

                for attr in attrs.into_iter() {
                    if !attr.path().is_ident("validate_check") {
                        field.attrs.push(attr);
                        continue;
                    }

                    let args = attr.parse_args::<Args>()?;
                    let invert = if args.invert { quote!(!) } else { quote!() };
                    let code = args.code.map(|code| quote!(code = #code,));
                    let message = args.message.map(|message| quote!(message = #message,));

                    let doc = quote!(#attr).to_string();
                    field.attrs.push(parse_quote!(#[doc = #doc]));

                    let check = args.check;
                    field.attrs.push(parse_quote! {
                        #[validate_inline_function(inline_function = {
                            if #invert #check {
                                ::core::result::Result::Ok(())
                            } else {
                                ::core::result::Result::Err(
                                    ::validator::ValidationError::new(#field_ident_str)
                                )
                            }
                        }, #code #message)]
                    });
                }
            }

            Ok(quote!(#item))
        }
        _ => bail_spanned!(fields, "expected named fields"),
    }
}

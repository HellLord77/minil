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

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct Args {
    ident: String,
    #[darling(multiple)]
    #[darling(rename = "input")]
    inputs: Vec<Expr>,
    code: Option<String>,
    message: Option<String>,
}

pub(super) fn expand(mut item: ItemStruct) -> syn::Result<TokenStream> {
    let ItemStruct {
        ref mut attrs,
        ref mut fields,
        ..
    } = item;
    attrs.push(parse_quote!(#[::validator_extra::validate_check]));

    match fields {
        Fields::Named(fields) => {
            for field in fields.named.iter_mut() {
                let field_ident = field.ident.as_ref().unwrap_or_else(|| unreachable!());
                let attrs = mem::take(&mut field.attrs);

                for attr in attrs.into_iter() {
                    if !attr.path().is_ident("validate_check_ass_fn") {
                        field.attrs.push(attr);
                        continue;
                    }

                    let doc = quote!(#attr).to_string();
                    field.attrs.push(parse_quote!(#[doc = #doc]));

                    let args = attr.parse_args::<Args>()?;
                    let code = args.code.map(|code| quote!(code = #code,));
                    let message = args.message.map(|message| quote!(message = #message,));

                    let ident = format_ident!("{}", args.ident);
                    let inputs = &args.inputs;
                    field.attrs.push(parse_quote! {
                        #[validate_check(check = #field_ident.#ident(#(#inputs),*), #code #message)]
                    });
                }
            }

            Ok(quote!(#item))
        }
        _ => bail_spanned!(fields, "expected named fields"),
    }
}

use std::mem;

use darling::FromMeta;
use darling::util::PreservedStrExpr;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Fields;
use syn::ItemStruct;
use syn::parse_quote;
use syn_utils::bail_spanned;

use crate::check;

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct Args {
    ident: String,
    #[darling(multiple)]
    #[darling(rename = "input")]
    inputs: Vec<PreservedStrExpr>,
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

pub(super) fn expand(mut item: ItemStruct) -> syn::Result<TokenStream> {
    let ItemStruct { ref mut fields, .. } = item;

    match fields {
        Fields::Named(fields) => {
            for field in fields.named.iter_mut() {
                let attrs = mem::take(&mut field.attrs);

                for attr in attrs.into_iter() {
                    if !attr.path().is_ident("validate_check_fn") {
                        field.attrs.push(attr);
                        continue;
                    }

                    let doc = format!("<!-- {} -->", quote!(#attr));
                    field.attrs.push(parse_quote!(#[doc = #doc]));

                    let args = attr.parse_args::<Args>()?;
                    let invert = args.invert.map(|invert| quote!(invert = #invert,));
                    let code = args.code.map(|code| quote!(code = #code,));
                    let message = args.message.map(|message| quote!(message = #message,));

                    let ident = format_ident!("{}", args.ident);
                    let inputs = &args.inputs;
                    field.attrs.push(parse_quote! {
                        #[validate_check(check = #ident(#(#inputs),*), #invert #code #message)]
                    });
                }
            }

            check::expand(item)
        }
        _ => bail_spanned!(fields, "expected named fields"),
    }
}

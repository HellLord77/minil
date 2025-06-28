use proc_macro2::TokenStream;
use quote::quote;
use syn::Fields;
use syn::ItemStruct;
use syn::parse_quote;
use syn_utils::field_has_attribute;
use syn_utils::parse_attrs;

use crate::attr::SerdeRenameChainAttrs;

pub(super) fn expand(
    args: SerdeRenameChainAttrs,
    mut item: ItemStruct,
) -> syn::Result<TokenStream> {
    let SerdeRenameChainAttrs { mut renamers } = args;

    let ItemStruct {
        ref mut attrs,
        ref mut fields,
        ..
    } = item;
    parse_attrs::<SerdeRenameChainAttrs>("serde_rename_chain", attrs)
        .map(|args| renamers.extend(args.renamers))?;

    let derive_attr = parse_quote!(#[derive(::serde_rename_chain::_SerdeRenameChain)]);
    attrs.push(derive_attr);

    match fields {
        Fields::Named(fields) => {
            for field in fields.named.iter_mut() {
                if field_has_attribute(field, "serde", "rename") {
                    continue;
                }

                let args =
                    parse_attrs::<SerdeRenameChainAttrs>("serde_rename_chain", &field.attrs)?;
                let renamers_iter = if args.renamers.is_empty() {
                    renamers.iter()
                } else {
                    args.renamers.iter()
                };

                let rename = renamers_iter.fold(
                    field
                        .ident
                        .clone()
                        .unwrap_or_else(|| unreachable!())
                        .to_string(),
                    |acc, renamer| renamer.apply(&acc),
                );

                let rename_attr = parse_quote!(#[serde(rename = #rename)]);
                field.attrs.push(rename_attr);
            }
            Ok(quote! { #item })
        }
        _ => Err(syn::Error::new_spanned(fields, "expected named fields")),
    }
}

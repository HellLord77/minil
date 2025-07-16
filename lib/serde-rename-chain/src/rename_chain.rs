use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Attribute;
use syn::Fields;
use syn::Item;
use syn::ItemEnum;
use syn::ItemStruct;
use syn::parse_quote;
use syn_utils::bail_spanned;
use syn_utils::has_attribute;
use syn_utils::parse_attrs;
use syn_utils::remove_attribute;

use crate::attr::SerdeRenameChainAttrs;
use crate::renamer::Renamer;

pub(super) fn expand(args: SerdeRenameChainAttrs, item: Item) -> syn::Result<TokenStream> {
    let SerdeRenameChainAttrs { mut renamers } = args;

    match item {
        Item::Struct(mut item) => {
            let ItemStruct {
                ref mut attrs,
                ref mut fields,
                ..
            } = item;
            process(&mut renamers, attrs)?;

            match fields {
                Fields::Named(fields) => {
                    for field in fields.named.iter_mut() {
                        apply(
                            &renamers,
                            field.ident.as_ref().unwrap_or_else(|| unreachable!()),
                            &mut field.attrs,
                        )?;
                    }

                    Ok(quote!(#item))
                }
                _ => bail_spanned!(fields, "expected named fields"),
            }
        }
        Item::Enum(mut item) => {
            let ItemEnum {
                ref mut attrs,
                ref mut variants,
                ..
            } = item;
            process(&mut renamers, attrs)?;

            for variant in variants.iter_mut() {
                apply(&renamers, &variant.ident, &mut variant.attrs)?;
            }

            Ok(quote!(#item))
        }
        _ => bail_spanned!(item, "expected struct or enum"),
    }
}

fn process(renamers: &mut Vec<Renamer>, attrs: &mut [Attribute]) -> syn::Result<()> {
    if has_attribute(attrs, "serde", "rename_all") {
        renamers.clear();
    } else {
        let args = parse_attrs::<SerdeRenameChainAttrs>("serde_rename_chain", attrs)?;
        renamers.extend(args.renamers);
    }

    Ok(())
}

fn apply(renamers: &[Renamer], ident: &Ident, attrs: &mut Vec<Attribute>) -> syn::Result<bool> {
    if has_attribute(attrs, "serde_rename_chain", "skip")
        || has_attribute(attrs, "serde", "skip")
        || (has_attribute(attrs, "serde", "skip_serializing")
            && has_attribute(attrs, "serde", "skip_deserializing"))
        || has_attribute(attrs, "serde", "rename")
    {
        return Ok(false);
    }

    let args = parse_attrs::<SerdeRenameChainAttrs>("serde_rename_chain", attrs)?;
    remove_attribute(attrs, "serde_rename_chain");

    let renamers = if args.renamers.is_empty() {
        renamers
    } else {
        &args.renamers
    };
    if renamers.is_empty() {
        return Ok(false);
    }

    #[cfg(debug_assertions)]
    {
        let doc = format!("<!-- {renamers:?} -->");
        attrs.push(parse_quote!(#[doc = #doc]));
    }

    let rename = renamers
        .iter()
        .fold(ident.to_string(), |acc, renamer| renamer.apply(&acc));
    attrs.push(parse_quote!(#[serde(rename = #rename)]));

    Ok(true)
}

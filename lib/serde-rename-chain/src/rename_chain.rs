use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;
use syn::ExprLit;
use syn::Fields;
use syn::Item;
use syn::ItemStruct;
use syn::Lit;
use syn::Meta;
use syn::MetaNameValue;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn_utils::bail_spanned;
use syn_utils::field_has_attribute;

use crate::error::TryNewErrorKind;
use crate::renamer::Renamer;
use crate::renamer::TryIntoRenamer;

fn parse(args: Punctuated<Meta, Comma>) -> syn::Result<Vec<Renamer>> {
    let mut renamers = vec![];

    for arg in args {
        match arg {
            Meta::NameValue(MetaNameValue { path, value, .. }) => match value {
                Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) => {
                    match (path.get_ident().unwrap().to_string(), lit_str.value())
                        .try_into_renamer()
                    {
                        Ok(renamer) => renamers.push(renamer),
                        Err(err) => {
                            if err.kind().is_renamer() {
                                bail_spanned!(path, err);
                            } else {
                                bail_spanned!(lit_str, err);
                            }
                        }
                    };
                }
                _ => bail_spanned!(value, "expected string literal"),
            },
            _ => {
                bail_spanned!(arg, "expected name-value pair");
            }
        };
    }

    Ok(renamers)
}

pub(super) fn expand(args: Punctuated<Meta, Comma>, item: Item) -> syn::Result<TokenStream> {
    let renamers = parse(args)?;

    match item {
        Item::Struct(mut item) => {
            let ItemStruct { ref mut fields, .. } = item;

            match fields {
                Fields::Named(fields) => {
                    for field in fields.named.iter_mut() {
                        if field_has_attribute(field, "serde", "rename") {
                            continue;
                        }

                        let rename = renamers
                            .iter()
                            .fold(field.ident.clone().unwrap().to_string(), |acc, renamer| {
                                renamer.apply(&acc)
                            });

                        field.attrs.push(parse_quote!( #[serde(rename = #rename)] ));
                    }
                    Ok(quote! { #item })
                }
                _ => Err(syn::Error::new_spanned(fields, "expected named fields")),
            }
        }
        _ => Err(syn::Error::new_spanned(item, "expected struct")),
    }
}

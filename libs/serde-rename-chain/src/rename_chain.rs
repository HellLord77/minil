use crate::{error::RenamerError, renamer::Renamer};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Error, Expr, ExprLit, Fields, Item, Lit, Meta, MetaNameValue, parse_quote,
    punctuated::Punctuated, token::Comma,
};

fn parse_rename_with_args(args: Punctuated<Meta, Comma>) -> Result<Vec<Renamer>, Error> {
    let mut renamers = vec![];

    for arg in args {
        match arg {
            Meta::NameValue(MetaNameValue {
                path,
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit_str),
                        ..
                    }),
                ..
            }) => {
                match Renamer::try_from_arg(
                    &path.get_ident().unwrap().to_string(),
                    lit_str.value().as_str(),
                ) {
                    Ok(renamer) => renamers.push(renamer),
                    Err(err @ RenamerError::Name(_)) => {
                        return Err(Error::new_spanned(path, err.to_string()));
                    }
                    Err(err @ RenamerError::Value(_)) => {
                        return Err(Error::new_spanned(lit_str, err.to_string()));
                    }
                };
            }
            _ => {
                return Err(Error::new_spanned(arg, "Expected name = \"value\""));
            }
        };
    }

    Ok(renamers)
}

pub(crate) fn rename_chain_impl(
    args: Punctuated<Meta, Comma>,
    mut item: Item,
) -> Result<TokenStream, Error> {
    let renamers = parse_rename_with_args(args)?;

    match item {
        Item::Struct(ref mut item_struct) => match &mut item_struct.fields {
            Fields::Named(fields) => {
                for field in fields.named.iter_mut() {
                    let has_rename = field.attrs.iter().any(|attr| {
                        if attr.path().is_ident("serde") {
                            if let Ok(Meta::NameValue(named_value)) = attr.parse_args() {
                                return named_value.path.is_ident("rename");
                            }
                        }
                        false
                    });
                    if has_rename {
                        continue;
                    }

                    let mut rename_lit = field.ident.clone().unwrap().to_string();
                    for renamer in &renamers {
                        rename_lit = renamer.apply(&rename_lit);
                    }
                    field
                        .attrs
                        .push(parse_quote!( #[serde(rename = #rename_lit)] ));
                }
            }
            _ => {
                return Err(Error::new_spanned(
                    &item_struct.fields,
                    "Expected named fields",
                ));
            }
        },
        _ => {
            return Err(Error::new_spanned(item, "Expected struct"));
        }
    }

    Ok(item.into_token_stream())
}

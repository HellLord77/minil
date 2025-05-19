use crate::renamer::{Renamer, RenamerError};
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
                    Err(RenamerError::Name) => {
                        let mut message = "Expected one of: add_prefix, \
                            add_suffix, strip_prefix, strip_suffix"
                            .to_owned();
                        if cfg!(feature = "convert_case") {
                            message += ", convert_case";
                        }
                        if cfg!(feature = "heck") {
                            message += ", heck";
                        }
                        if cfg!(feature = "inflector") {
                            message += ", inflector";
                        }
                        return Err(Error::new_spanned(path, message));
                    }
                    #[cfg(feature = "convert_case")]
                    Err(RenamerError::ConvertCase) => {
                        let mut message = "Expected one of: snake, constant, upper_snake, \
                            ada, kebab, cobol, upper_kebab, train, flat, upper_flat, pascal, \
                            upper_camel, lower, upper, title, sentence, alternating, toggle"
                            .to_owned();
                        if cfg!(feature = "convert_case_random") {
                            message += ", random, pseudo_random";
                        }
                        return Err(Error::new_spanned(lit_str, message));
                    }
                    #[cfg(feature = "heck")]
                    Err(RenamerError::Heck) => {
                        let message = "Expected one of: kebab, lower_camel, shouty_kebab, shouty_snake, \
                            shouty_snek, snake, snek, title, train, upper_camel, pascal";
                        return Err(Error::new_spanned(lit_str, message));
                    }
                    #[cfg(feature = "inflector")]
                    Err(RenamerError::Inflector) => {
                        let mut message =
                            "Expected one of: camel, pascal, snake, screaming_snake, \
                            kebab, train, sentence, title, foreign_key"
                                .to_owned();
                        if cfg!(feature = "inflector_heavyweight") {
                            message += ", class, table, plural, singular";
                        }
                        return Err(Error::new_spanned(lit_str, message));
                    }
                };
            }
            Meta::NameValue(MetaNameValue {
                path,
                value: Expr::Path(expr_path),
                ..
            }) if path.is_ident("custom_fn") => {
                dbg!(&expr_path.to_token_stream());
                todo!();
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

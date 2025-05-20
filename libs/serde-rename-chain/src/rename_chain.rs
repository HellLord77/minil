use crate::{error::RenamerError, renamer::Renamer};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{
    Expr, ExprLit, Lit, Meta, MetaNameValue, parse_quote, punctuated::Punctuated, spanned::Spanned,
    token::Comma,
};
use syn_utils::{apply_function_to_struct_fields, bail, field_has_attribute};

fn renamers_from_args(args: Punctuated<Meta, Comma>) -> syn::Result<Vec<Renamer>> {
    let mut renamers = vec![];

    for arg in args {
        match arg {
            Meta::NameValue(MetaNameValue {
                path,
                value: Expr::Path(expr_path),
                ..
            }) if path.is_ident("crabtime") => {
                dbg!(&expr_path.to_token_stream());
                // todo!();
            }
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
                    Err(err) => {
                        let tokens = match err {
                            RenamerError::Name(_) => path.into_token_stream(),
                            RenamerError::Value(_) => lit_str.into_token_stream(),
                        };
                        bail!(tokens.span(), err);
                    }
                };
            }
            _ => {
                bail!(arg.span(), "expected a named argument");
            }
        };
    }

    Ok(renamers)
}

pub(super) fn rename_all_chain_impl(
    args: Punctuated<Meta, Comma>,
    input: TokenStream,
) -> syn::Result<TokenStream2> {
    let renamers = renamers_from_args(args)?;

    apply_function_to_struct_fields(input, |field| {
        if field_has_attribute(field, "serde", "rename") {
            return Ok(());
        }

        let rename_lit = renamers.iter().fold(
            field
                .ident
                .clone()
                .ok_or("expected named field")?
                .to_string(),
            |rename_lit, renamer| renamer.apply(&rename_lit),
        );

        field
            .attrs
            .push(parse_quote!( #[serde(rename = #rename_lit)] ));
        Ok(())
    })
}

use crate::renamer::Renamer;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::Expr;
use syn::ExprLit;
use syn::Lit;
use syn::Meta;
use syn::MetaNameValue;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn_utils::apply_function_to_struct_fields;
use syn_utils::bail;
use syn_utils::field_has_attribute;

fn renamers_from_args(args: Punctuated<Meta, Comma>) -> syn::Result<Vec<Renamer>> {
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
                    Err(err) => {
                        let tokens = match err {
                            crate::Error::Name(_) => path.into_token_stream(),
                            crate::Error::Value(..) => lit_str.into_token_stream(),
                        };
                        bail!(tokens.span(), err);
                    }
                };
            }
            _ => {
                bail!(arg.span(), "expected named argument");
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

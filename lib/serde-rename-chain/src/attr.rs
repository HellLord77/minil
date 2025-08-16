use syn::Expr;
use syn::ExprLit;
use syn::Lit;
use syn::Meta;
use syn::MetaNameValue;
use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn_utils::Combine;
use syn_utils::bail_spanned;

use crate::renamer::Renamer;

#[derive(Default)]
pub(super) struct SerdeRenameChainAttrs {
    pub(super) renamers: Vec<Renamer>,
}

impl Parse for SerdeRenameChainAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;
        let mut renamers = vec![];

        for arg in args {
            match arg {
                Meta::NameValue(MetaNameValue { path, value, .. }) => match value {
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit_str),
                        ..
                    }) => {
                        let renamer = Renamer::try_from((
                            path.get_ident().unwrap().to_string(),
                            lit_str.value(),
                        ));
                        match renamer {
                            Ok(renamer) => renamers.push(renamer),
                            Err(err) => {
                                if err.is_renamer() {
                                    bail_spanned!(path, err);
                                } else {
                                    bail_spanned!(lit_str, err);
                                }
                            }
                        }
                    }
                    _ => bail_spanned!(value, "expected string literal"),
                },
                _ => bail_spanned!(arg, "expected name-value pair"),
            }
        }

        Ok(Self { renamers })
    }
}

impl Combine for SerdeRenameChainAttrs {
    fn combine(mut self, other: Self) -> syn::Result<Self> {
        self.renamers.extend(other.renamers);
        Ok(self)
    }
}

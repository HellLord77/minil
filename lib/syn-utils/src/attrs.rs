use syn::Attribute;
use syn::Meta;
use syn::Token;
use syn::parse::Parser;
use syn::parse_quote;
use syn::punctuated::Punctuated;

pub fn has_attribute(attrs: &[Attribute], namespace: &str, name: &str) -> bool {
    for attr in attrs {
        if attr.path().is_ident(namespace) {
            if let Meta::List(expr) = &attr.meta {
                let nested = match Punctuated::<Meta, Token![,]>::parse_terminated
                    .parse2(expr.tokens.clone())
                {
                    Ok(nested) => nested,
                    Err(_) => continue,
                };

                for expr in nested {
                    match expr {
                        Meta::NameValue(expr) => {
                            if let Some(ident) = expr.path.get_ident() {
                                if *ident == name {
                                    return true;
                                }
                            }
                        }
                        Meta::Path(expr) => {
                            if let Some(ident) = expr.get_ident() {
                                if *ident == name {
                                    return true;
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    false
}

pub fn remove_derive_attribute(attrs: &mut Vec<Attribute>, name: &str) {
    attrs.retain_mut(|attr| {
        if attr.path().is_ident("derive") {
            if let Meta::List(expr) = &attr.meta {
                if let Ok(nested) =
                    Punctuated::<Meta, Token![,]>::parse_terminated.parse2(expr.tokens.clone())
                {
                    let filtered = nested
                        .iter()
                        .filter(|meta| {
                            if let Meta::Path(path) = meta {
                                !path.is_ident(name)
                            } else {
                                true
                            }
                        })
                        .collect::<Vec<_>>();

                    if filtered.is_empty() {
                        return false;
                    }

                    *attr = parse_quote!(#[derive(#(#filtered),*)]);
                }
            }
        }
        true
    });
}

pub fn remove_attribute(attrs: &mut Vec<Attribute>, namespace: &str) {
    attrs.retain(|attr| !attr.path().is_ident(namespace));
}

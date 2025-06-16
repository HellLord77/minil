use syn::Field;
use syn::Meta;
use syn::Token;
use syn::parse::Parser;
use syn::punctuated::Punctuated;

pub fn field_has_attribute(field: &Field, namespace: &str, name: &str) -> bool {
    for attr in &field.attrs {
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

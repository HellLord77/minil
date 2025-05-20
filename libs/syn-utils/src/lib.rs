use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    __private::TokenStream, Field, Fields, ItemEnum, ItemStruct, Meta, Token, parse, parse::Parser,
    punctuated::Punctuated, spanned::Spanned,
};

trait IteratorExt {
    fn collect_error(self) -> syn::Result<()>
    where
        Self: Iterator<Item = syn::Result<()>> + Sized,
    {
        let accu = Ok(());
        self.fold(accu, |accu, error| match (accu, error) {
            (Ok(()), error) => error,
            (accu, Ok(())) => accu,
            (Err(mut err), Err(error)) => {
                err.combine(error);
                Err(err)
            }
        })
    }
}

impl<I> IteratorExt for I where I: Iterator<Item = syn::Result<()>> + Sized {}

fn apply_on_fields<F>(fields: &mut Fields, function: F) -> syn::Result<()>
where
    F: Fn(&mut Field) -> Result<(), String>,
{
    match fields {
        Fields::Unit => Ok(()),
        Fields::Named(fields) => fields
            .named
            .iter_mut()
            .map(|field| function(field).map_err(|err| syn::Error::new(field.span(), err)))
            .collect_error(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter_mut()
            .map(|field| function(field).map_err(|err| syn::Error::new(field.span(), err)))
            .collect_error(),
    }
}

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

pub fn apply_function_to_struct_fields<F>(
    input: TokenStream,
    function: F,
) -> syn::Result<TokenStream2>
where
    F: Copy,
    F: Fn(&mut Field) -> Result<(), String>,
{
    if let Ok(mut input) = parse::<ItemStruct>(input.clone()) {
        apply_on_fields(&mut input.fields, function)?;
        Ok(quote!(#input))
    } else {
        Err(syn::Error::new(Span::call_site(), "expected struct"))
    }
}

pub fn apply_function_to_enum_fields<F>(
    input: TokenStream,
    function: F,
) -> syn::Result<TokenStream2>
where
    F: Copy,
    F: Fn(&mut Field) -> Result<(), String>,
{
    if let Ok(mut input) = parse::<ItemEnum>(input) {
        input
            .variants
            .iter_mut()
            .map(|variant| apply_on_fields(&mut variant.fields, function))
            .collect_error()?;
        Ok(quote!(#input))
    } else {
        Err(syn::Error::new(Span::call_site(), "expected enum"))
    }
}

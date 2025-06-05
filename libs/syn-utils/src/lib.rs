mod attr;
mod ty;

pub use attr::Combine;
pub use attr::combine_attribute;
pub use attr::parse_assignment_attribute;
pub use attr::parse_attrs;
pub use attr::parse_parenthesized_attribute;

pub use ty::peel_option;
pub use ty::peel_result_ok;

use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use quote::quote;
use syn::__private::TokenStream;
use syn::Field;
use syn::Fields;
use syn::ItemEnum;
use syn::ItemStruct;
use syn::Meta;
use syn::Token;
use syn::parse;
use syn::parse::Parse;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

#[macro_export]
macro_rules! bail {
    ($span:expr, $message:literal $(,)?) => {
        return std::result::Result::Err(syn::Error::new($span, $message))
    };
    ($span:expr, $err:expr $(,)?) => {
        return std::result::Result::Err(syn::Error::new($span, $err))
    };
    ($span:expr, $fmt:expr, $($arg:tt)*) => {
        return std::result::Result::Err(syn::Error::new($span, std::format!($fmt, $($arg)*)))
    };
}

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

#[deprecated]
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

#[deprecated]
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
        bail!(Span::call_site(), "expected struct")
    }
}

#[deprecated]
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
        bail!(Span::call_site(), "expected enum");
    }
}

#[deprecated]
pub fn into_macro_output(input: syn::Result<TokenStream2>) -> TokenStream {
    input.unwrap_or_else(|err| err.to_compile_error()).into()
}

pub fn expand_with<F, I, K>(input: TokenStream, f: F) -> TokenStream
where
    F: FnOnce(I) -> syn::Result<K>,
    I: Parse,
    K: ToTokens,
{
    expand(parse(input).and_then(f))
}

pub fn expand_attr_with<F, A, I, K>(attr: TokenStream, input: TokenStream, f: F) -> TokenStream
where
    F: FnOnce(A, I) -> K,
    A: Parse,
    I: Parse,
    K: ToTokens,
{
    let expand_result = (|| {
        let attr = parse(attr)?;
        let input = parse(input)?;
        Ok(f(attr, input))
    })();
    expand(expand_result)
}

pub fn expand<T>(result: syn::Result<T>) -> TokenStream
where
    T: ToTokens,
{
    match result {
        Ok(tokens) => {
            let tokens = quote! { #tokens }.into();
            if std::env::var_os("SYN_UTILS_DEBUG").is_some() {
                eprintln!("{tokens}");
            }
            tokens
        }
        Err(err) => err.into_compile_error().into(),
    }
}

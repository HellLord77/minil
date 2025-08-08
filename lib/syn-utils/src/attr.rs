use std::any::type_name;

use quote::ToTokens;
use syn::Attribute;
use syn::Token;
use syn::parenthesized;
use syn::parse::Parse;
use syn::parse::ParseStream;

pub fn parse_parenthesized_attribute<K, T>(
    input: ParseStream<'_>,
    out: &mut Option<(K, T)>,
) -> syn::Result<()>
where
    K: Parse + ToTokens,
    T: Parse,
{
    let kw = input.parse()?;

    let content;
    parenthesized!(content in input);
    let inner = content.parse()?;

    if out.is_some() {
        let kw_name = type_name::<K>().split("::").last().unwrap();
        let msg = format!("`{kw_name}` specified more than once");
        return Err(syn::Error::new_spanned(kw, msg));
    }

    *out = Some((kw, inner));
    Ok(())
}

pub fn parse_assignment_attribute<K, T>(
    input: ParseStream<'_>,
    out: &mut Option<(K, T)>,
) -> syn::Result<()>
where
    K: Parse + ToTokens,
    T: Parse,
{
    let kw = input.parse()?;
    input.parse::<Token![=]>()?;
    let inner = input.parse()?;

    if out.is_some() {
        let kw_name = type_name::<K>().split("::").last().unwrap();
        let msg = format!("`{kw_name}` specified more than once");
        return Err(syn::Error::new_spanned(kw, msg));
    }

    *out = Some((kw, inner));
    Ok(())
}

pub trait Combine: Sized {
    fn combine(self, other: Self) -> syn::Result<Self>;
}

pub fn parse_attrs<T>(ident: &str, attrs: &[Attribute]) -> syn::Result<T>
where
    T: Default + Parse + Combine,
{
    attrs
        .iter()
        .filter(|attr| attr.meta.path().is_ident(ident))
        .map(Attribute::parse_args)
        .try_fold(T::default(), |out, next| out.combine(next?))
}

pub fn combine_attribute<K, T>(a: &mut Option<(K, T)>, b: Option<(K, T)>) -> syn::Result<()>
where
    K: ToTokens,
{
    if let Some((kw, inner)) = b {
        if a.is_some() {
            let kw_name = type_name::<K>().split("::").last().unwrap();
            let msg = format!("`{kw_name}` specified more than once");
            return Err(syn::Error::new_spanned(kw, msg));
        }
        *a = Some((kw, inner));
    }
    Ok(())
}

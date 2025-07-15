use std::mem;

use darling::FromMeta;
use derive_more::Into;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;
use syn::Fields;
use syn::ItemStruct;
use syn::parse_quote;
use syn_utils::bail_spanned;

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct Args {
    #[darling(multiple)]
    is_empty: Vec<IsEmptyArgs>,

    #[darling(multiple)]
    is_ascii: Vec<IsAsciiArgs>,

    #[darling(multiple)]
    is_some: Vec<IsSomeArgs>,

    #[darling(multiple)]
    is_none: Vec<IsNoneArgs>,

    #[darling(multiple)]
    is_ok: Vec<IsOkArgs>,

    #[darling(multiple)]
    is_err: Vec<IsErrArgs>,

    #[darling(multiple)]
    exists: Vec<ExistsArgs>,

    #[darling(multiple)]
    is_file: Vec<IsFileArgs>,

    #[darling(multiple)]
    is_dir: Vec<IsDirArgs>,

    #[darling(multiple)]
    eq: Vec<EqArgs>,

    #[darling(multiple)]
    ne: Vec<NeArgs>,

    #[darling(multiple)]
    lt: Vec<LtArgs>,

    #[darling(multiple)]
    le: Vec<LeArgs>,

    #[darling(multiple)]
    gt: Vec<GtArgs>,

    #[darling(multiple)]
    ge: Vec<GeArgs>,

    #[darling(multiple)]
    starts_with: Vec<StartsWithArgs>,

    #[darling(multiple)]
    ends_with: Vec<EndsWithArgs>,

    #[darling(multiple)]
    contains: Vec<ContainsArgs>,

    #[darling(multiple)]
    contains_key: Vec<ContainsKeyArgs>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct IsEmptyArgs {
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct IsAsciiArgs {
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct IsSomeArgs {
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct IsNoneArgs {
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct IsOkArgs {
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct IsErrArgs {
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct ExistsArgs {
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct IsFileArgs {
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct IsDirArgs {
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct EqArgs {
    other: Expr,
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct NeArgs {
    other: Expr,
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct LtArgs {
    other: Expr,
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct LeArgs {
    other: Expr,
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct GtArgs {
    other: Expr,
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct GeArgs {
    other: Expr,
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct ContainsArgs {
    pattern: Expr,
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct StartsWithArgs {
    pattern: Expr,
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct EndsWithArgs {
    pattern: Expr,
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Into, FromMeta)]
#[darling(derive_syn_parse)]
struct ContainsKeyArgs {
    pattern: Expr,
    invert: Option<bool>,
    code: Option<String>,
    message: Option<String>,
}

macro_rules! impl_check_ass_fn {
    ($args:ident, $ident:ident, $($inputs:ident),* $(,)?) => {{
        let mut attrs = ::std::vec![];
        let ident = ::core::stringify!($ident);

        for arg in $args.$ident {
            let doc = ::std::format!("{arg:?}");
            attrs.push(::syn::parse_quote!(#[doc = #doc]));

            let ($($inputs,)* invert, code, message) = arg.into();
            let invert = invert.map(|invert| ::quote::quote!(invert = #invert,));
            let code = code.map(|code| ::quote::quote!(code = #code,));
            let message = message.map(|message| ::quote::quote!(message = #message,));

            attrs.push(::syn::parse_quote! {
                #[validate_check_ass_fn(ident = #ident, $(input = #$inputs,)* #invert #code #message)]
            });
        }

        attrs
    }}
}

pub(super) fn expand(mut item: ItemStruct) -> syn::Result<TokenStream> {
    let ItemStruct {
        ref mut attrs,
        ref mut fields,
        ..
    } = item;

    match fields {
        Fields::Named(fields) => {
            attrs.push(parse_quote!(#[::validator_extra::validate_check_fn]));
            attrs.push(parse_quote!(#[::validator_extra::validate_check_ass_fn]));

            for field in fields.named.iter_mut() {
                let attrs = mem::take(&mut field.attrs);

                for attr in attrs.into_iter() {
                    if !attr.path().is_ident("validate_extra") {
                        field.attrs.push(attr);
                        continue;
                    }
                    let args = attr.parse_args::<Args>()?;

                    field.attrs.extend(impl_check_ass_fn!(args, is_empty,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_ascii,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_some,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_none,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_ok,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_err,));
                    field.attrs.extend(impl_check_ass_fn!(args, exists,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_file,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_dir,));

                    field.attrs.extend(impl_check_ass_fn!(args, eq, input0));
                    field.attrs.extend(impl_check_ass_fn!(args, ne, input0));
                    field.attrs.extend(impl_check_ass_fn!(args, lt, input0));
                    field.attrs.extend(impl_check_ass_fn!(args, le, input0));
                    field.attrs.extend(impl_check_ass_fn!(args, gt, input0));
                    field.attrs.extend(impl_check_ass_fn!(args, ge, input0));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, starts_with, input0));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, ends_with, input0));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, contains, input0));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, contains_key, input0));
                }
            }

            Ok(quote!(#item))
        }
        _ => bail_spanned!(fields, "expected named fields"),
    }
}

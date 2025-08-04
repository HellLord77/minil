use std::mem;

use proc_macro2::TokenStream;
use syn::Fields;
use syn::ItemStruct;
use syn_utils::bail_spanned;

use crate::check_ass_fn;

macro_rules! define_args {
    ($($ident:ident$(($($inputs:ident),* $(,)?))?),* $(,)?) => {::paste::paste! {
        #[derive(::core::fmt::Debug, ::darling::FromMeta)]
        #[darling(derive_syn_parse)]
        struct Args {$(
            #[darling(multiple)]
            $ident: ::std::vec::Vec<[<$ident:camel Args>]>,
        )*}

        impl Args {
            fn into_attrs(self, attrs: &mut ::std::vec::Vec<::syn::Attribute>) {
                $(
                    for args in self.$ident {
                        args.into_attrs(attrs);
                    }
                )*
            }
        }

        $(
            #[derive(::core::fmt::Debug, ::darling::FromMeta)]
            #[darling(derive_syn_parse)]
            struct [<$ident:camel Args>] {
                $($($inputs: ::darling::util::PreservedStrExpr,)*)*
                invert: ::core::option::Option<bool>,
                code: ::core::option::Option<String>,
                message: ::core::option::Option<String>,
            }

            impl [<$ident:camel Args>] {
                fn into_attrs(self, attrs: &mut ::std::vec::Vec<::syn::Attribute>) {
                    $($(let $inputs = self.$inputs;)*)*

                    let invert = self.invert.map(|invert| ::quote::quote!(invert = #invert,));
                    let code = self.code.map(|code| ::quote::quote!(code = #code,));
                    let message = self.message.map(|message| ::quote::quote!(message = #message,));

                    let attr = ::quote::quote! {
                        #[validate_extra($ident($($(input = #$inputs,)*)* #invert #code #message))]
                    };
                    let doc = ::std::format!("<!-- {attr} -->");
                    attrs.push(::syn::parse_quote!(#[doc = #doc]));

                    let ident_str = ::core::stringify!($ident);
                    attrs.push(::syn::parse_quote! {
                        #[validate_check_ass_fn(ident = #ident_str, $($(input = #$inputs,)*)* #invert #code #message)]
                    });
                }
            }
        )*
    }};
}

define_args! {
    // int
    is_positive,
    is_negative,

    // uint
    is_power_of_two,

    // float
    is_nan,
    is_infinite,
    is_finite,
    is_subnormal,
    is_normal,
    is_sign_positive,
    is_sign_negative,

    // char
    is_alphabetic,
    is_lowercase,
    is_uppercase,
    is_whitespace,
    is_alphanumeric,
    is_control,
    is_numeric,
    is_ascii,
    is_ascii_alphabetic,
    is_ascii_uppercase,
    is_ascii_lowercase,
    is_ascii_alphanumeric,
    is_ascii_digit,
    is_ascii_octdigit,
    is_ascii_hexdigit,
    is_ascii_punctuation,
    is_ascii_graphic,
    is_ascii_whitespace,
    is_ascii_control,

    // option
    is_some,
    is_none,

    // result
    is_ok,
    is_err,

    // str
    is_empty,

    // path
    exists,
    is_file,
    is_dir,

    // partial_eq
    eq(other),
    ne(other),

    // partial_ord
    lt(other),
    le(other),
    gt(other),
    ge(other),

    // unsigned
    is_multiply_of(rhs),

    // char
    is_digit(radix),
    eq_ignore_ascii_case(other),

    // str
    is_char_boundary(index),
    contains(pattern),
    starts_with(pattern),
    ends_with(pattern),

    // hash_map
    contains_key(pattern),
}

pub(super) fn expand(mut item: ItemStruct) -> syn::Result<TokenStream> {
    let ItemStruct { ref mut fields, .. } = item;

    match fields {
        Fields::Named(fields) => {
            for field in &mut fields.named {
                let attrs = mem::take(&mut field.attrs);

                for attr in attrs {
                    if !attr.path().is_ident("validate_extra") {
                        field.attrs.push(attr);
                        continue;
                    }

                    let args = attr.parse_args::<Args>()?;
                    args.into_attrs(&mut field.attrs);
                }
            }

            check_ass_fn::expand(item)
        }
        _ => bail_spanned!(fields, "expected named fields"),
    }
}

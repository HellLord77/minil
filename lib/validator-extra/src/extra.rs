use std::mem;

use darling::util::PreservedStrExpr;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Fields;
use syn::ItemStruct;
use syn::parse_quote;
use syn_utils::bail_spanned;

macro_rules! define_base_args {
    ($($ident:ident => $args:ident),* $(,)?) => {
        #[derive(::core::fmt::Debug, ::darling::FromMeta)]
        #[darling(derive_syn_parse)]
        struct Args {
            $(
                #[darling(multiple)]
                $ident: ::std::vec::Vec<$args>,
            )*
        }
    };
}

define_base_args! {
    is_positive => IsPositiveArgs,
    is_negative => IsNegativeArgs,
    is_power_of_two => IsPowerOfTwoArgs,
    is_nan => IsNaNArgs,
    is_infinite => IsInfiniteArgs,
    is_finite => IsFiniteArgs,
    is_subnormal => IsSubnormalArgs,
    is_normal => IsNormalArgs,
    is_sign_positive => IsSignPositiveArgs,
    is_sign_negative => IsSignNegativeArgs,
    is_alphabetic => IsAlphabeticArgs,
    is_lowercase => IsLowercaseArgs,
    is_uppercase => IsUppercaseArgs,
    is_whitespace => IsWhitespaceArgs,
    is_alphanumeric => IsAlphanumericArgs,
    is_control => IsControlArgs,
    is_numeric => IsNumericArgs,
    is_ascii => IsAsciiArgs,
    is_ascii_alphabetic => IsAsciiAlphabeticArgs,
    is_ascii_uppercase => IsAsciiUppercaseArgs,
    is_ascii_lowercase => IsAsciiLowercaseArgs,
    is_ascii_alphanumeric => IsAsciiAlphanumericArgs,
    is_ascii_digit => IsAsciiDigitArgs,
    is_ascii_octdigit => IsAsciiOctdigitArgs,
    is_ascii_hexdigit => IsAsciiHexdigitArgs,
    is_ascii_punctuation => IsAsciiPunctuationArgs,
    is_ascii_graphic => IsAsciiGraphicArgs,
    is_ascii_whitespace => IsAsciiWhitespaceArgs,
    is_ascii_control => IsAsciiControlArgs,
    is_some => IsSomeArgs,
    is_none => IsNoneArgs,
    is_ok => IsOkArgs,
    is_err => IsErrArgs,
    is_empty => IsEmptyArgs,
    exists => ExistsArgs,
    is_file => IsFileArgs,
    is_dir => IsDirArgs,

    eq => EqArgs,
    ne => NeArgs,
    lt => LtArgs,
    le => LeArgs,
    gt => GtArgs,
    ge => GeArgs,
    is_multiply_of => IsMultiplyOfArgs,
    is_digit => IsDigitArgs,
    eq_ignore_ascii_case => EqIgnoreAsciiCaseArgs,
    is_char_boundary => IsCharBoundaryArgs,
    contains => ContainsArgs,
    starts_with => StartsWithArgs,
    ends_with => EndsWithArgs,
    contains_key => ContainsKeyArgs,
}

macro_rules! define_args {
    ($ident:ident, $($inputs:ident),* $(,)?) => {
        #[derive(::core::fmt::Debug, ::darling::FromMeta)]
        #[darling(derive_syn_parse)]
        struct $ident {
            $($inputs: PreservedStrExpr,)*
            invert: Option<bool>,
            code: Option<String>,
            message: Option<String>,
        }
    };
}

// int
define_args!(IsPositiveArgs,);
define_args!(IsNegativeArgs,);

// uint
define_args!(IsPowerOfTwoArgs,);

// float
define_args!(IsNaNArgs,);
define_args!(IsInfiniteArgs,);
define_args!(IsFiniteArgs,);
define_args!(IsSubnormalArgs,);
define_args!(IsNormalArgs,);
define_args!(IsSignPositiveArgs,);
define_args!(IsSignNegativeArgs,);

// char
define_args!(IsAlphabeticArgs,);
define_args!(IsLowercaseArgs,);
define_args!(IsUppercaseArgs,);
define_args!(IsWhitespaceArgs,);
define_args!(IsAlphanumericArgs,);
define_args!(IsControlArgs,);
define_args!(IsNumericArgs,);
define_args!(IsAsciiArgs,);
define_args!(IsAsciiAlphabeticArgs,);
define_args!(IsAsciiUppercaseArgs,);
define_args!(IsAsciiLowercaseArgs,);
define_args!(IsAsciiAlphanumericArgs,);
define_args!(IsAsciiDigitArgs,);
define_args!(IsAsciiOctdigitArgs,);
define_args!(IsAsciiHexdigitArgs,);
define_args!(IsAsciiPunctuationArgs,);
define_args!(IsAsciiGraphicArgs,);
define_args!(IsAsciiWhitespaceArgs,);
define_args!(IsAsciiControlArgs,);

// Option
define_args!(IsSomeArgs,);
define_args!(IsNoneArgs,);

// Result
define_args!(IsOkArgs,);
define_args!(IsErrArgs,);

// str
define_args!(IsEmptyArgs,);

// Path
define_args!(ExistsArgs,);
define_args!(IsFileArgs,);
define_args!(IsDirArgs,);

// PartialEq
define_args!(EqArgs, other);
define_args!(NeArgs, other);

// PartialOrd
define_args!(LtArgs, other);
define_args!(LeArgs, other);
define_args!(GtArgs, other);
define_args!(GeArgs, other);

// unsigned
define_args!(IsMultiplyOfArgs, rhs);

// char
define_args!(IsDigitArgs, radix);
define_args!(EqIgnoreAsciiCaseArgs, other);

// str
define_args!(IsCharBoundaryArgs, index);
define_args!(ContainsArgs, pattern);
define_args!(StartsWithArgs, pattern);
define_args!(EndsWithArgs, pattern);

// HashMap
define_args!(ContainsKeyArgs, pattern);

macro_rules! impl_check_ass_fn {
    ($args:ident, $ident:ident, $($inputs:ident),* $(,)?) => {{
        let mut attrs = ::std::vec![];
        let ident = ::core::stringify!($ident);

        for arg in $args.$ident {
            let doc = ::std::format!("{arg:?}");
            attrs.push(::syn::parse_quote!(#[doc = #doc]));

            $(let $inputs = arg.$inputs;)*

            let invert = arg.invert.map(|invert| ::quote::quote!(invert = #invert,));
            let code = arg.code.map(|code| ::quote::quote!(code = #code,));
            let message = arg.message.map(|message| ::quote::quote!(message = #message,));

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
            attrs.insert(0, parse_quote!(#[::validator_extra::validate_check_fn]));
            attrs.insert(0, parse_quote!(#[::validator_extra::validate_check_ass_fn]));

            for field in fields.named.iter_mut() {
                let attrs = mem::take(&mut field.attrs);

                for attr in attrs.into_iter() {
                    if !attr.path().is_ident("validate_extra") {
                        field.attrs.push(attr);
                        continue;
                    }
                    let args = attr.parse_args::<Args>()?;

                    field.attrs.extend(impl_check_ass_fn!(args, is_positive,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_negative,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_power_of_two,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_nan,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_infinite,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_finite,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_subnormal,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_normal,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_sign_positive,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_sign_negative,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_alphabetic,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_lowercase,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_uppercase,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_whitespace,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_alphanumeric,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_control,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_numeric,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_ascii,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_ascii_alphabetic,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_ascii_uppercase,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_ascii_lowercase,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_ascii_alphanumeric,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_ascii_digit,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_ascii_octdigit,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_ascii_hexdigit,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_ascii_punctuation,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_ascii_graphic,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_ascii_whitespace,));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_ascii_control,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_some,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_none,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_ok,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_err,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_empty,));
                    field.attrs.extend(impl_check_ass_fn!(args, exists,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_file,));
                    field.attrs.extend(impl_check_ass_fn!(args, is_dir,));

                    field.attrs.extend(impl_check_ass_fn!(args, eq, other));
                    field.attrs.extend(impl_check_ass_fn!(args, ne, other));
                    field.attrs.extend(impl_check_ass_fn!(args, lt, other));
                    field.attrs.extend(impl_check_ass_fn!(args, le, other));
                    field.attrs.extend(impl_check_ass_fn!(args, gt, other));
                    field.attrs.extend(impl_check_ass_fn!(args, ge, other));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_multiply_of, rhs));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_digit, radix));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, eq_ignore_ascii_case, other));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, is_char_boundary, index));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, contains, pattern));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, starts_with, pattern));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, ends_with, pattern));
                    field
                        .attrs
                        .extend(impl_check_ass_fn!(args, contains_key, pattern));
                }
            }

            Ok(quote!(#item))
        }
        _ => bail_spanned!(fields, "expected named fields"),
    }
}

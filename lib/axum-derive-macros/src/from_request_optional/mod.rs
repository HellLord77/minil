mod attr;
mod tr;

use attr::FromRequestOptionalAttrs;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use quote::quote_spanned;
use syn::Field;
use syn::Fields;
use syn::Index;
use syn::Item;
use syn::ItemStruct;
use syn::Member;
use syn::Path;
use syn::Type;
use syn::Visibility;
use syn::parse_quote;
use syn::parse2;
use syn::spanned::Spanned;
use syn_utils::bail_spanned;
use syn_utils::parse_attrs;
use syn_utils::peel_option;
use syn_utils::remove_attribute;
use syn_utils::remove_derive_attribute;
pub(super) use tr::Trait;

use crate::from_request_optional::attr::kw;

pub(super) fn modify(item: Item, tr: Trait) -> syn::Result<TokenStream> {
    match item {
        Item::Struct(mut item) => {
            let ItemStruct {
                ref mut attrs,
                ref ident,
                ..
            } = item;

            let derive_attrs = match tr {
                Trait::FromRequestOptional => {
                    parse_quote!(#[derive(
                        ::axum_derive_macros::FromRequest,
                        ::axum_derive_macros::_FromRequestOptional
                    )])
                }
                Trait::FromRequestPartsOptional => {
                    parse_quote!(#[derive(
                        ::axum_derive_macros::FromRequestParts,
                        ::axum_derive_macros::_FromRequestPartsOptional
                    )])
                }
            };
            attrs.push(derive_attrs);

            let form_request_via = format_ident!("_FromRequestOptional{ident}");
            let from_request_attr = parse_quote!(#[from_request(via(#form_request_via))]);
            attrs.push(from_request_attr);

            Ok(quote! { #item })
        }
        Item::Enum(_item) => unimplemented!(),
        _ => bail_spanned!(item, "expected struct or enum"),
    }
}

pub(super) fn expand(item: Item, tr: Trait) -> syn::Result<TokenStream> {
    match item {
        Item::Struct(mut item) => {
            let ItemStruct {
                ref mut attrs,
                ref mut vis,
                ident,
                ref mut fields,
                ..
            } = item;
            *vis = Visibility::Inherited;
            item.ident = format_ident!("_FromRequestOptional{ident}");

            let remove_attrs = match tr {
                Trait::FromRequestOptional => [
                    "::axum_derive_macros::FromRequest",
                    "::axum_derive_macros::_FromRequestOptional",
                ],
                Trait::FromRequestPartsOptional => [
                    "::axum_derive_macros::FromRequestParts",
                    "::axum_derive_macros::_FromRequestPartsOptional",
                ],
            };
            for attr in remove_attrs {
                remove_derive_attribute(attrs, attr);
            }
            remove_attribute(attrs, "from_request");

            let derive_attr = match tr {
                Trait::FromRequestOptional => {
                    parse_quote!(
                        #[derive(::axum::extract::FromRequest)]
                    )
                }
                Trait::FromRequestPartsOptional => {
                    parse_quote!(
                        #[derive(::axum::extract::FromRequestParts)]
                    )
                }
            };
            attrs.push(derive_attr);

            let optional_ident = &item.ident;
            let extract_fields = extract_fields(fields)?;

            let impl_from = quote! {
                #[automatically_derived]
                impl ::std::convert::From<#optional_ident> for #ident {
                    fn from(value: #optional_ident) -> Self {
                        Self {
                            #(#extract_fields)*
                        }
                    }
                }
            };

            Ok(quote! {
                #item
                #impl_from
            })
        }
        Item::Enum(_item) => unimplemented!(),
        _ => bail_spanned!(item, "expected struct or enum"),
    }
}

fn extract_fields(fields: &mut Fields) -> syn::Result<Vec<TokenStream>> {
    fn member(field: &Field, index: usize) -> TokenStream {
        match &field.ident {
            Some(ident) => quote! { #ident },
            None => {
                let member = Member::Unnamed(Index {
                    index: index as u32,
                    span: field.span(),
                });
                quote! { #member }
            }
        }
    }

    fn into_inner(
        optional_via: &Option<(attr::kw::via, Path)>,
        via: &Option<(attr::kw::via, Path)>,
        ty_span: Span,
    ) -> TokenStream {
        if let Some((_, optional_path)) = optional_via {
            if let Some((_, path)) = via {
                let span = path.span();
                quote_spanned! {span=>
                    .0.map(|#path(inner)| inner)
                }
            } else {
                let optional_span = optional_path.span();
                quote_spanned! {optional_span=>
                    .0
                }
            }
        } else {
            quote_spanned! {ty_span=>}
        }
    }

    fn into_outer(
        optional_via: &Option<(kw::via, Path)>,
        via: &Option<(kw::via, Path)>,
        ty_span: Span,
        field_ty: &Type,
    ) -> TokenStream {
        if let Some((_, optional_path)) = optional_via {
            if let Some((_, path)) = via {
                let span = path.span();
                quote_spanned! {span=>
                    #optional_path<#path<#field_ty>>
                }
            } else {
                let inner_span = field_ty.span();
                quote_spanned! {inner_span=>
                    #optional_path<#field_ty>
                }
            }
        } else {
            quote_spanned! {ty_span=>
                #field_ty
            }
        }
    }

    fields
        .iter_mut()
        .enumerate()
        .map(|(index, field)| {
            let Field { attrs, ty, .. } = field;
            let ty_span = ty.span();

            let optional_via =
                parse_attrs::<FromRequestOptionalAttrs>("from_request_optional", attrs)?.via;
            remove_attribute(attrs, "from_request_optional");

            let via = if optional_via.is_some() {
                let via = parse_attrs::<FromRequestOptionalAttrs>("from_request", attrs)?.via;
                remove_attribute(attrs, "from_request");

                let inner_ty = peel_option(ty)
                    .ok_or_else(|| syn::Error::new(ty_span, "expected `Option<T>`"))?;
                let field_ty = into_outer(&optional_via, &via, ty_span, inner_ty);
                *ty = parse2(field_ty)?;

                via
            } else {
                None
            };

            let member = member(&field, index);
            let into_inner = into_inner(&optional_via, &via, ty_span);

            Ok(quote_spanned! {ty_span=>
                #member: value.#member #into_inner,
            })
        })
        .collect()
}

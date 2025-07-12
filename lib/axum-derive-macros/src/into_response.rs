use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::quote_spanned;
use syn::Field;
use syn::Fields;
use syn::Index;
use syn::Item;
use syn::ItemStruct;
use syn::Member;
use syn::Path;
use syn::spanned::Spanned;
use syn_utils::bail_spanned;
use syn_utils::parse_attrs;

use crate::attr::Attrs;
use crate::attr::kw;

pub(super) fn expand(item: Item, parts: bool) -> syn::Result<TokenStream> {
    match item {
        Item::Struct(item) => {
            let ItemStruct { ident, fields, .. } = item;
            let extract_fields = extract_fields(&fields, &parts)?;

            Ok(if parts {
                quote! {
                    #[automatically_derived]
                    impl ::axum::response::IntoResponseParts for #ident {
                        type Error = ::axum::response::Response;

                        fn into_response_parts(
                            self,
                            res: ::axum::response::ResponseParts
                        ) -> ::core::result::Result<::axum::response::ResponseParts, Self::Error> {
                            #(#extract_fields)*
                        }
                    }
                }
            } else {
                quote! {
                    #[automatically_derived]
                    impl ::axum::response::IntoResponse for #ident {
                        fn into_response(self) -> ::axum::response::Response {
                            ::axum::response::IntoResponse::into_response((#(#extract_fields)*))
                        }
                    }
                }
            })
        }
        Item::Enum(_item) => unimplemented!(),
        _ => bail_spanned!(item, "expected struct or enum"),
    }
}

fn extract_fields(fields: &Fields, parts: &bool) -> syn::Result<Vec<TokenStream>> {
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
        via: &Option<(kw::via, Path)>,
        member: TokenStream,
        ty_span: Span,
    ) -> TokenStream {
        if let Some((_, path)) = via {
            let span = path.span();
            quote_spanned! {span=>
                #path(self.#member)
            }
        } else {
            quote_spanned! {ty_span=>
                self.#member
            }
        }
    }

    let mut fields_iter = fields.iter();
    let last = fields_iter.next_back();

    let mut extract_fields = fields_iter
        .enumerate()
        .map(|(index, field)| {
            let Attrs { via } = parse_attrs("into_response", &field.attrs)?;

            let member = member(field, index);
            let ty_span = field.ty.span();
            let into_inner = into_inner(&via, member, ty_span);

            let tokens = if *parts {
                quote_spanned! {ty_span=>
                    let res = ::axum::response::IntoResponseParts::into_response_parts(#into_inner, res)
                        .map_err(::axum::response::IntoResponse::into_response)?;
                }
            } else {
                quote_spanned! {ty_span=>
                    #into_inner,
                }
            };
            Ok(tokens)
        })
        .collect::<syn::Result<Vec<_>>>()?;

    if let Some(field) = last {
        let Attrs { via } = parse_attrs("into_response", &field.attrs)?;

        let member = member(field, fields.len() - 1);
        let ty_span = field.ty.span();
        let into_inner = into_inner(&via, member, ty_span);

        let tokens = if *parts {
            quote_spanned! {ty_span=>
                ::axum::response::IntoResponseParts::into_response_parts(#into_inner, res)
                    .map_err(::axum::response::IntoResponse::into_response)
            }
        } else {
            quote_spanned! {ty_span=>
                #into_inner
            }
        };
        extract_fields.push(tokens);
    };

    Ok(extract_fields)
}

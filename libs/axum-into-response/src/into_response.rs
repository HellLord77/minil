use crate::attr::IntoResponseAttrs;
use crate::attr::kw;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::quote_spanned;
use syn::Field;
use syn::Index;
use syn::Item;
use syn::ItemStruct;
use syn::Member;
use syn::Path;
use syn::spanned::Spanned;
use syn_utils::parse_attrs;
use tr::Trait;

pub(crate) mod tr {
    pub(crate) enum Trait {
        IntoResponse,
        IntoResponseParts,
    }
}

pub(super) fn expand(item: Item, tr: Trait) -> syn::Result<TokenStream> {
    match item {
        Item::Struct(item) => {
            let ItemStruct { ident, fields, .. } = item;

            fn member(field: &Field, index: usize) -> TokenStream {
                match &field.ident {
                    Some(ident) => quote! { #ident },
                    _ => {
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
            let last = match tr {
                Trait::IntoResponse => None,
                Trait::IntoResponseParts => fields_iter.next_back(),
            };

            let mut extract_fields = fields_iter
                .enumerate()
                .map(|(index, field)| {
                    let IntoResponseAttrs { via } = parse_attrs("into_response", &field.attrs)?;

                    let member = member(field, index);
                    let ty_span = field.ty.span();
                    let into_inner = into_inner(&via, member, ty_span);

                    let tokens = match tr {
                        Trait::IntoResponse => quote_spanned! {ty_span=>
                            #into_inner,
                        },
                        Trait::IntoResponseParts => quote_spanned! {ty_span=>
                            let res = ::axum::response::IntoResponseParts::into_response_parts(#into_inner, res)
                                .map_err(::axum::response::IntoResponse::into_response)?;
                        }
                    };
                    Ok(tokens)
                })
                .collect::<syn::Result<Vec<_>>>()?;

            if let Some(field) = last {
                let IntoResponseAttrs { via } = parse_attrs("into_response", &field.attrs)?;

                let member = member(field, fields.len() - 1);
                let ty_span = field.ty.span();
                let into_inner = into_inner(&via, member, ty_span);

                let tokens = quote_spanned! {ty_span=>
                    ::axum::response::IntoResponseParts::into_response_parts(#into_inner, res)
                        .map_err(::axum::response::IntoResponse::into_response)
                };
                extract_fields.push(tokens);
            };

            Ok(match tr {
                Trait::IntoResponse => quote! {
                    #[automatically_derived]
                    impl ::axum::response::IntoResponse for #ident {
                        fn into_response(self) -> ::axum::response::Response {
                            ::axum::response::IntoResponse::into_response((#(#extract_fields)*))
                        }
                    }
                },
                Trait::IntoResponseParts => quote! {
                    #[automatically_derived]
                    impl ::axum::response::IntoResponseParts for #ident {
                        type Error = ::axum::response::Response;

                        fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
                            #(#extract_fields)*
                        }
                    }
                },
            })
        }
        Item::Enum(_item) => {
            unimplemented!()
        }
        _ => Err(syn::Error::new_spanned(item, "expected `struct` or `enum`")),
    }
}

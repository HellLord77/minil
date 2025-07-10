use syn::Path;
use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn_utils::Combine;
use syn_utils::combine_attribute;
use syn_utils::parse_parenthesized_attribute;

pub(super) mod kw {
    use syn::custom_keyword;

    custom_keyword!(via);
}

#[derive(Default)]
pub(super) struct Attrs {
    pub(super) via: Option<(kw::via, Path)>,
}

impl Parse for Attrs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut via = None;

        while !input.is_empty() {
            let lh = input.lookahead1();
            if lh.peek(kw::via) {
                parse_parenthesized_attribute(input, &mut via)?;
            } else {
                return Err(lh.error());
            }

            let _ = input.parse::<Token![,]>();
        }

        Ok(Self { via })
    }
}

impl Combine for Attrs {
    fn combine(mut self, other: Self) -> syn::Result<Self> {
        let Self { via } = other;
        combine_attribute(&mut self.via, via)?;
        Ok(self)
    }
}

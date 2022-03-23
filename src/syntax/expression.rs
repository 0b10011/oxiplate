use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Identifier(&'a str),
}

impl ToTokens for Expression<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Expression::Identifier(identifier) => {
                let identifier = syn::Ident::new(&identifier, proc_macro2::Span::call_site());
                quote! {write!(f, "{}", self.#identifier)?;}
            }
        });
    }
}

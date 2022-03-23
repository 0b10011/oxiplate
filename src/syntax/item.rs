use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use super::Writ;
use super::Statement;
use super::Static;

#[derive(Debug, PartialEq)]
pub enum Item<'a> {
    Comment,
    Writ(Writ<'a>),
    Statement(Statement<'a>),
    Static(Static),
}

impl ToTokens for Item<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Item::Comment => quote! {},
            Item::Writ(writ) => quote! { #writ },
            Item::Statement(statement) => quote! { #statement },
            Item::Static(text) => quote! { #text },
        });
    }
}

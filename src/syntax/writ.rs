use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use super::{Expression, Item};

#[derive(Debug, PartialEq)]
pub struct Writ<'a>(pub Expression<'a>);

impl<'a> From<Writ<'a>> for Item<'a> {
    fn from(writ: Writ<'a>) -> Self {
        Item::Writ(writ)
    }
}

impl ToTokens for Writ<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expression = &self.0;
        tokens.append_all(quote! { #expression });
    }
}

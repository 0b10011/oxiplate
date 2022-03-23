use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use super::Item;

#[derive(Debug, PartialEq)]
pub struct Template<'a>(pub Vec<Item<'a>>);

impl ToTokens for Template<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for item in &self.0 {
            tokens.append_all(quote! { #item });
        }
    }
}
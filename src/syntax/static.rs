use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use super::Item;

#[derive(Debug, PartialEq)]
pub struct Static(pub String);

impl<'a> From<Static> for Item<'a> {
    fn from(r#static: Static) -> Self {
        Item::Static(r#static)
    }
}

impl ToTokens for Static {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let text = &self.0;
        tokens.append_all(quote! {write!(f, "{}", #text)?;});
    }
}

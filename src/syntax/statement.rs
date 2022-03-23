use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use super::Item;

#[derive(Debug, PartialEq)]
pub struct Statement<'a>(pub &'a str);

impl<'a> From<Statement<'a>> for Item<'a> {
    fn from(statement: Statement<'a>) -> Self {
        Item::Statement(statement)
    }
}

impl ToTokens for Statement<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expression = &self.0;
        tokens.append_all(quote! {write!(f, "{}", #expression)?;});
    }
}

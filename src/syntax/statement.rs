use super::{Item, Res, Span, Static};
use nom::bytes::complete::tag;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, PartialEq, Eq)]
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

pub(super) fn statement(input: Span) -> Res<&str, (Item, Option<Static>)> {
    let (input, output) = tag("test")(input)?;

    let whitespace = None;

    Ok((input, (Statement(output.fragment()).into(), whitespace)))
}

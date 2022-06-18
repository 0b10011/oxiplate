use super::{
    expression::expression, item::tag_end, template::is_whitespace, Expression, Item, Res, Static,
};
use crate::Source;
use nom::bytes::complete::take_while;
use nom::combinator::cut;
use nom::error::context;
use nom::sequence::preceded;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Writ<'a>(pub Expression<'a>);

impl<'a> From<Writ<'a>> for Item<'a> {
    fn from(writ: Writ<'a>) -> Self {
        Item::Writ(writ)
    }
}

impl ToTokens for Writ<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expression = &self.0;
        tokens.append_all(quote! { write!(f, "{}", #expression)?; });
    }
}

pub(super) fn writ(input: Source) -> Res<Source, (Item, Option<Static>)> {
    let (input, _) = take_while(is_whitespace)(input)?;
    let (input, output) = context("Expected an expression.", cut(expression))(input)?;
    let (input, trailing_whitespace) = context(
        "Expecting the writ tag to be closed with `_}}`, `-}}`, or `}}`.",
        cut(preceded(take_while(is_whitespace), cut(tag_end("}}")))),
    )(input)?;

    Ok((input, (Writ(output).into(), trailing_whitespace)))
}

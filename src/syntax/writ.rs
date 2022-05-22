use super::{
    expression::expression, item::tag_end, template::is_whitespace, Expression, Item, Res, Span,
    Static,
};
use nom::bytes::complete::take_while;
use nom::combinator::opt;
use nom::sequence::preceded;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, PartialEq, Eq)]
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

pub(super) fn writ(input: Span) -> Res<&str, (Item, Option<Static>)> {
    let (input, _) = opt(take_while(is_whitespace))(input)?;
    let (input, output) = expression(input)?;
    let (input, trailing_whitespace) =
        preceded(opt(take_while(is_whitespace)), tag_end("}}"))(input)?;

    Ok((input, (Writ(output).into(), trailing_whitespace)))
}

use std::marker::PhantomData;

use super::item::Item;
use super::prelude::*;

pub(in crate::config::parser) fn newline<'a>() -> NewlineParser<'a> {
    NewlineParser {
        phantom_data: PhantomData,
    }
}
pub(in crate::config::parser) struct NewlineParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for NewlineParser<'a> {
    type TokenKind = TokenKind;
    type Output = Item<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, newline) = take(TokenKind::Newline).parse(tokens)?;

        Ok((tokens, Item::Newline(newline.source().clone())))
    }
}

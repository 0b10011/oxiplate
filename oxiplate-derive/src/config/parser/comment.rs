use std::marker::PhantomData;

use super::item::Item;
use super::newline_or_eof::newline_or_eof;
use crate::config::parser::prelude::*;

pub(in crate::config::parser) fn comment<'a>() -> CommentParser<'a> {
    CommentParser {
        phantom_data: PhantomData,
    }
}
pub(in crate::config::parser) struct CommentParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for CommentParser<'a> {
    type TokenKind = TokenKind;
    type Output = Item<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (comment, newline)) = (
            take(TokenKind::Comment),
            cut(
                "Newline or end of file expected after comment",
                newline_or_eof(),
            ),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            Item::Comment(
                comment
                    .source()
                    .clone()
                    .merge(newline, "Newline expected after comment"),
            ),
        ))
    }
}

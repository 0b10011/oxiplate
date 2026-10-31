use std::marker::PhantomData;

use super::comment::comment;
use super::expression::{Expression, expression};
use super::newline::newline;
use super::prelude::*;
use super::table::table;

pub(in crate::config::parser) enum Item<'a> {
    Comment(Source<'a>),
    Expression(Expression<'a>),
    Newline(Source<'a>),
}

impl<'a> Item<'a> {
    pub(in crate::config::parser) fn parse() -> ItemParser<'a> {
        ItemParser {
            phantom_data: PhantomData,
        }
    }

    pub(in crate::config::parser) fn source(&self) -> &Source<'a> {
        match self {
            Self::Comment(source) | Self::Newline(source) => source,
            Self::Expression(expression) => &expression.source(),
        }
    }
}

pub(in crate::config::parser) struct ItemParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for ItemParser<'a> {
    type TokenKind = TokenKind;
    type Output = Item<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        alt((expression(), table(), comment(), newline())).parse(tokens)
    }
}

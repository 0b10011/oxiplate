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
    fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((expression(), table(), comment(), newline())).parse(tokens)
    }

    fn source(&self) -> &Source<'a> {
        match self {
            Self::Comment(source) | Self::Newline(source) => source,
            Self::Expression(expression) => &expression.source(),
        }
    }
}

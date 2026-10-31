use std::marker::PhantomData;

use super::item::Item;
use super::newline_or_eof::newline_or_eof;
use super::prelude::*;
use super::string::string;
use super::value::{Value, value};

pub(in crate::config::parser) struct Expression<'a> {
    ancestor_keys: Vec<Key<'a>>,
    key: Key<'a>,
    value: Option<Value<'a>>,
    source: Source<'a>,
}

impl<'a> Expression<'a> {
    pub(in crate::config::parser) fn new(
        ancestor_keys: Vec<Key<'a>>,
        key: Key<'a>,
        value: Option<Value<'a>>,
        source: Source<'a>,
    ) -> Self {
        Self {
            ancestor_keys,
            key,
            value,
            source,
        }
    }

    pub fn ancestor_keys(&self) -> &Vec<Key<'a>> {
        &self.ancestor_keys
    }

    pub fn key(&self) -> &Key<'a> {
        &self.key
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn take_value(self) -> Option<Value<'a>> {
        self.value
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }
}

pub(in crate::config::parser) fn expression<'a>() -> ExpressionParser<'a> {
    ExpressionParser {
        phantom_data: PhantomData,
    }
}
pub(in crate::config::parser) struct ExpressionParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for ExpressionParser<'a> {
    type TokenKind = TokenKind;
    type Output = Item<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (ancestors, key, equal, value, comment, newline)): (
            _,
            (Vec<(Key, _)>, Key, _, _, _, _),
        ) = (
            many0((into(string()), take(TokenKind::DotSeparator))),
            into(string()),
            cut("`=` expected after key", take(TokenKind::Equal)),
            cut("Boolean or string value expected after `=`", value()),
            ignore_recoverable_errors(take(TokenKind::Comment)),
            cut(
                "Newline or end of file expected after expression",
                newline_or_eof(),
            ),
        )
            .parse(tokens)?;

        let mut ancestor_keys = Vec::with_capacity(ancestors.len());
        let mut ancestor_source = None;
        for (ancestor, dot) in ancestors {
            ancestor_source = Some(
                ancestor
                    .source()
                    .append_to_some(
                        ancestor_source,
                        "Ancestor expected after previous ancestory",
                    )
                    .merge(dot.source(), "`.` expected after ancestor"),
            );

            ancestor_keys.push(ancestor);
        }

        let source = key
            .source()
            .clone()
            .append_to_some(ancestor_source, "Key expected after ancestors")
            .merge(equal.source(), "`=` expected after key`")
            .merge(value.source(), "Value expected after `=`")
            .merge_some(comment.map(Token::source), "Comment expected after value")
            .merge(newline, "Newline or end of file expected after comment");

        let expression = Expression {
            ancestor_keys,
            key,
            value: Some(value),
            source,
        };

        Ok((tokens, Item::Expression(expression)))
    }
}

pub(in crate::config::parser) struct Key<'a> {
    value: &'a str,
    source: &'a Source<'a>,
}

impl<'a> Key<'a> {
    pub(in crate::config::parser) fn new(value: &'a str, source: &'a Source<'a>) -> Self {
        Self { value, source }
    }

    pub(crate) fn value(&self) -> &'a str {
        self.value
    }

    pub(crate) fn source(&self) -> &'a Source<'a> {
        self.source
    }
}

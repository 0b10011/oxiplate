use std::collections::HashMap;

use super::expression::{Expression, Key};
use super::item::Item;
use super::newline_or_eof::newline_or_eof;
use super::prelude::*;
use super::string::string;
use super::table_or_value::TableOrValue;

pub(in crate::config::parser) type Table<'a> = HashMap<&'a str, TableOrValue<'a>>;

pub(in crate::config::parser) fn table<'a>(tokens: TokenSlice<'a>) -> Res<'a, Item<'a>> {
    let (tokens, (open_bracket, ancestors, key, close_bracket, newline)): (
        _,
        (_, Vec<(Key, _)>, Key, _, _),
    ) = (
        take(TokenKind::BracketOpen),
        many0((into(string()), take(TokenKind::DotSeparator))),
        into(string()),
        take(TokenKind::BracketClose),
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

    let source = open_bracket
        .source()
        .clone()
        .merge_some(ancestor_source.as_ref(), "Ancestors expected after `[`")
        .merge(key.source(), "Key expected after ancestors")
        .merge(close_bracket.source(), "`]` expected after key")
        .merge(newline, "Newline or end of file expected after comment");

    let expression = Expression {
        ancestor_keys,
        key,
        value: None,
        source,
    };

    Ok((tokens, Item::Expression(expression)))
}

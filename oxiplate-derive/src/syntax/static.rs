use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_while, take_while1};
use nom::combinator::{eof, fail, peek, recognize};
use nom::multi::many_till;
use quote::quote_spanned;

use super::item::tag_start;
use super::template::{adjusted_whitespace, is_whitespace};
use super::{Item, Res};
use crate::{BuiltTokens, Source};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Static<'a>(pub &'a str, pub Source<'a>);

impl Static<'_> {
    pub fn to_token(&self) -> BuiltTokens {
        let text = &self.0;
        let span = self.1.span();
        (quote_spanned! { span => #text }, text.len())
    }
}

/// Type of static text.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StaticType {
    /// One or more whitespace characters.
    /// Tracked separately to allow for it
    /// to appear in the top level when extending templates.
    Whitespace,

    /// Plain text that may contain whitespace.
    Text,
}

pub(crate) fn parse_static(input: Source) -> Res<Source, Vec<Item>> {
    let (input, (output, _)) = many_till(
        alt((
            take_till1(is_whitespace_or_opening_brace),
            take_while1(is_whitespace),
            // The opening brace needs to be matched separately
            // to avoid skipping over tags (writs, statements, and comments).
            tag("{"),
        )),
        peek(alt((
            recognize(tag_start),
            recognize(adjusted_whitespace),
            eof,
        ))),
    )
    .parse(input)?;

    // Must be checked for many0() call will fail due to infinite loop
    if output.is_empty() {
        return fail().parse(input);
    }

    let mut items: Vec<Item> = vec![];
    let mut source: Option<Source> = None;
    let mut whitespace_only = true;
    for item in output {
        if item.range.len() == 1 && item.as_str() == "{" {
            if let Some(source_value) = source {
                items.push(Item::Static(
                    Static(source_value.as_str(), source_value),
                    if whitespace_only {
                        StaticType::Whitespace
                    } else {
                        StaticType::Text
                    },
                ));
                source = None;
                whitespace_only = true;
            }

            items.push(Item::Static(Static(item.as_str(), item), StaticType::Text));
            continue;
        }

        let is_whitespace = take_while(is_whitespace)
            .parse(item.clone())?
            .0
            .as_str()
            .is_empty();

        if !is_whitespace {
            whitespace_only = false;
        }

        if let Some(source) = &mut source {
            source.range.end = item.range.end;
        } else {
            source = Some(item);
        }
    }

    if let Some(source) = source {
        items.push(Item::Static(
            Static(source.as_str(), source),
            if whitespace_only {
                StaticType::Whitespace
            } else {
                StaticType::Text
            },
        ));
    }

    Ok((input, items))
}

fn is_whitespace_or_opening_brace(char: char) -> bool {
    char == '{' || is_whitespace(char)
}

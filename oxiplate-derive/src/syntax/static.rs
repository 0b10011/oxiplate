use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_while, take_while1};
use nom::combinator::{eof, fail, peek, recognize};
use nom::multi::many_till;
use proc_macro2::TokenStream;
use quote::quote_spanned;

use super::item::tag_start;
use super::template::{adjusted_whitespace, is_whitespace};
use super::{Item, Res};
use crate::Source;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Static<'a>(pub &'a str, pub Source<'a>);

impl Static<'_> {
    pub fn to_token(&self) -> TokenStream {
        let text = &self.0;
        let span = self.1.span();
        quote_spanned! { span => #text }
    }
}

pub(crate) fn parse_static(input: Source) -> Res<Source, Vec<Item>> {
    let (input, (output, _)) = many_till(
        alt((
            take_till1(is_whitespace_or_brace),
            take_while1(is_whitespace),
            tag("{"),
        )),
        peek(alt((
            recognize(tag_start),
            recognize(adjusted_whitespace),
            eof,
        ))),
    )(input)?;

    // Must be checked for many0() call will fail due to infinite loop
    if output.is_empty() {
        return fail(input);
    }

    let mut source: Option<Source> = None;
    let mut whitespace_only = true;
    for item in output {
        let is_whitespace = take_while(is_whitespace)(item.clone())?
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

    let mut items: Vec<Item> = vec![];
    if let Some(source) = source {
        items.push(Item::Static(
            Static(source.as_str(), source),
            whitespace_only,
        ));
    }

    Ok((input, items))
}

fn is_whitespace_or_brace(char: char) -> bool {
    char == '{' || is_whitespace(char)
}

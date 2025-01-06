use super::{
    item::tag_start,
    template::{adjusted_whitespace, is_whitespace},
    Item, Res,
};
use crate::Source;
use nom::bytes::complete::{tag, take_till1, take_while1};
use nom::combinator::{eof, fail, peek, recognize};
use nom::multi::many_till;
use nom::{branch::alt, bytes::complete::take_while};
use proc_macro2::TokenStream;
use quote::quote_spanned;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Static<'a>(pub &'a str, pub Source<'a>);

impl Static<'_> {
    pub fn to_token(&self) -> TokenStream {
        let text = &self.0;
        let span = self.1.span();
        quote_spanned! { span => #text }
    }
}

impl<'a> From<Static<'a>> for Item<'a> {
    fn from(r#static: Static<'a>) -> Self {
        Item::Static(r#static)
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
    let mut leading_whitespace: Option<Source> = None;
    let mut trailing_whitespace: Option<Source> = None;
    let mut items = output.into_iter().peekable();
    while let Some(item) = items.next() {
        let is_whitespace = take_while(is_whitespace)(item.clone())?
            .0
            .as_str()
            .is_empty();

        // Check if leading whitespace
        if is_whitespace && source.is_none() {
            if let Some(leading_whitespace) = &mut leading_whitespace {
                leading_whitespace.range.end = item.range.end;
            } else {
                leading_whitespace = Some(item);
            }
            continue;
        }

        // Check if trailing whitespace
        if is_whitespace && items.peek().is_none() {
            trailing_whitespace = Some(item);
            continue;
        }

        if let Some(source) = &mut source {
            source.range.end = item.range.end;
        } else {
            source = Some(item);
        }
    }

    let mut items: Vec<Item> = vec![];
    if let Some(leading_whitespace) = leading_whitespace {
        items.push(Item::Whitespace(Static(
            leading_whitespace.as_str(),
            leading_whitespace,
        )));
    }
    if let Some(source) = source {
        items.push(Item::Static(Static(source.as_str(), source)));
    }
    if let Some(trailing_whitespace) = trailing_whitespace {
        items.push(Item::Whitespace(Static(
            trailing_whitespace.as_str(),
            trailing_whitespace,
        )));
    }

    Ok((input, items))
}

fn is_whitespace_or_brace(char: char) -> bool {
    char == '{' || is_whitespace(char)
}

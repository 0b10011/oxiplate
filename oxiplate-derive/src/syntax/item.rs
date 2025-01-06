use std::ops::RangeTo;

use super::{
    comment::comment, statement::statement, template::whitespace, writ::writ, Res, Statement,
    Static, Writ,
};
use crate::{Source, State};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, opt};
use nom::error::VerboseError;
use nom::sequence::tuple;
use nom::Slice as _;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

pub(super) enum ItemToken {
    StaticText(TokenStream),
    DynamicText(TokenStream),
    Comment,
    Statement(TokenStream),
}

#[derive(Debug)]
pub(crate) enum Item<'a> {
    Comment,
    Writ(Writ<'a>),
    Statement(Statement<'a>),
    Static(Static<'a>),
    Whitespace(Static<'a>),
    CompileError(String, Source<'a>),
}

impl Item<'_> {
    pub(super) fn to_token(&self) -> ItemToken {
        match self {
            Item::Comment => ItemToken::Comment,
            Item::Writ(writ) => ItemToken::DynamicText(writ.to_token()),
            Item::Statement(statement) => ItemToken::Statement(quote! { #statement }),
            Item::Static(text) => {
                // `{` and `}` are handled specially when formatting the text,
                // so any string that contains them needs to be treated as dynamic
                // to ensure it doesn't break string formatting.
                if text.0.contains(['{', '}']) {
                    ItemToken::DynamicText(text.to_token())
                } else {
                    ItemToken::StaticText(text.to_token())
                }
            }
            Item::Whitespace(whitespace) => ItemToken::StaticText(whitespace.to_token()),
            Item::CompileError(text, source) => {
                let span = source.slice(RangeTo { end: 1 }).span();
                ItemToken::Statement(quote_spanned! {span=> compile_error!(#text); })
            }
        }
    }
}

#[derive(Debug)]
pub enum TagOpen {
    Writ,
    Statement,
    Comment,
}

pub(crate) fn parse_tag<'a>(
    state: &'a State,
    should_output_blocks: &'a bool,
) -> impl Fn(Source) -> Res<Source, Vec<Item>> + 'a {
    |input| {
        let (input, (leading_whitespace, open)) = tag_start(input)?;

        let (input, (tag, trailing_whitespace)) = match open {
            TagOpen::Writ => cut(writ(state))(input)?,
            TagOpen::Statement => cut(statement(state, should_output_blocks))(input)?,
            TagOpen::Comment => cut(comment)(input)?,
        };

        let mut items = vec![];

        if let Some(leading_whitespace) = leading_whitespace {
            items.push(Item::Whitespace(leading_whitespace));
        }

        items.push(tag);

        if let Some(trailing_whitespace) = trailing_whitespace {
            items.push(Item::Whitespace(trailing_whitespace));
        }

        Ok((input, items))
    }
}

pub(crate) fn tag_start(input: Source) -> Res<Source, (Option<Static>, TagOpen)> {
    let (input, (whitespace, open, command)) = tuple((
        // Whitespace is optional, but tracked because it could be altered by tag.
        opt(whitespace),
        // Check if this is actually a tag; if it's not, that's fine, just return early.
        tag_open,
        // Whitespace control characters are optional.
        opt(alt((collapse_whitespace_command, trim_whitespace_command))),
    ))(input)?;

    let whitespace = match command {
        // Collapse to a single space if there's any leading whitespace.
        Some('_') => whitespace.map(|whitespace| Static(" ", whitespace)),
        // Remove any leading whitespace.
        Some('-') => None,
        Some(_) => unreachable!("Only - or _ should be matched"),
        // Convert any leading whitespace to `Static()` without adjusting.
        None => whitespace.map(|whitespace| Static(whitespace.as_str(), whitespace)),
    };

    Ok((input, (whitespace, open)))
}

pub(crate) fn tag_end<'a>(
    tag_close: &'static str,
) -> impl Fn(Source<'a>) -> Res<Source<'a>, Option<Static<'a>>> + 'a {
    move |input| {
        if let Ok((input, _tag)) = tag::<_, _, VerboseError<_>>(tag_close)(input.clone()) {
            return Ok((input, None));
        }

        let (input, (command, _, whitespace)) = tuple((
            alt((collapse_whitespace_command, trim_whitespace_command)),
            tag(tag_close),
            opt(whitespace),
        ))(input)?;

        let whitespace = match command {
            '_' => whitespace.map(|whitespace| Static(" ", whitespace)),
            '-' => None,
            _ => unreachable!("Only - or _ should be matched"),
        };

        Ok((input, whitespace))
    }
}

pub(crate) fn tag_open(input: Source) -> Res<Source, TagOpen> {
    let (input, output) = alt((
        tag("{{"), // writ
        tag("{%"), // statement
        tag("{#"), // comment
    ))(input)?;

    match output.as_str() {
        "{{" => Ok((input, TagOpen::Writ)),
        "{%" => Ok((input, TagOpen::Statement)),
        "{#" => Ok((input, TagOpen::Comment)),
        _ => panic!("This should never happen"),
    }
}

fn collapse_whitespace_command(input: Source) -> Res<Source, char> {
    char('_')(input)
}

fn trim_whitespace_command(input: Source) -> Res<Source, char> {
    char('-')(input)
}

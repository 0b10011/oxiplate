use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, opt, peek};
use nom::error::VerboseError;
use nom::sequence::{pair, tuple};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use super::comment::comment;
use super::r#static::StaticType;
use super::statement::statement;
use super::template::whitespace;
use super::writ::writ;
use super::{Res, Statement, Static, Writ};
use crate::{Source, State};

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

    /// Static text, with a boolean for whether the text is only whitespace.
    Static(Static<'a>, StaticType),
    Whitespace(Static<'a>),
    CompileError(String, Source<'a>),
}

impl Item<'_> {
    pub(super) fn to_token(&self) -> ItemToken {
        match self {
            Item::Comment => ItemToken::Comment,
            Item::Writ(writ) => ItemToken::DynamicText(writ.to_token()),
            Item::Statement(statement) => ItemToken::Statement(quote! { #statement }),
            Item::Static(text, static_type) => match static_type {
                StaticType::Brace => ItemToken::DynamicText(text.to_token()),
                StaticType::Whitespace | StaticType::Text => ItemToken::StaticText(text.to_token()),
            },
            Item::Whitespace(whitespace) => ItemToken::StaticText(whitespace.to_token()),
            Item::CompileError(text, source) => {
                let span = source.span();
                ItemToken::Statement(quote_spanned! {span=> compile_error!(#text); })
            }
        }
    }
}

#[derive(Debug)]
pub enum TagOpen<'a> {
    Writ(Source<'a>),
    Statement(Source<'a>),
    Comment(Source<'a>),
}

pub(crate) fn parse_tag<'a>(
    state: &'a State,
    is_extending: &'a bool,
) -> impl Fn(Source) -> Res<Source, Vec<Item>> + 'a {
    |input| {
        let (input, (leading_whitespace, open)) = tag_start(input)?;

        let (input, (tag, trailing_whitespace)) = match open {
            TagOpen::Writ(_source) => cut(writ(state))(input)?,
            TagOpen::Statement(_source) => cut(statement(state, is_extending))(input)?,
            TagOpen::Comment(_source) => cut(comment)(input)?,
        };

        let mut items = vec![];

        if let Some(leading_whitespace) = leading_whitespace {
            items.push(Item::Whitespace(leading_whitespace));
        }

        items.push(tag);

        if let Some(trailing_whitespace) = trailing_whitespace {
            items.push(trailing_whitespace);
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

    let whitespace = if let Some(command) = command {
        match command.as_str() {
            "_" => whitespace.map(|whitespace| Static(" ", whitespace)),
            "-" => None,
            _ => unreachable!("Only - or _ should be matched"),
        }
    } else {
        whitespace.map(|whitespace| Static(whitespace.as_str(), whitespace))
    };

    Ok((input, (whitespace, open)))
}

pub(crate) fn tag_end<'a>(
    tag_close: &'static str,
) -> impl Fn(Source<'a>) -> Res<Source<'a>, Option<Item<'a>>> + 'a {
    move |input| {
        if let Ok((input, _tag)) = tag::<_, _, VerboseError<_>>(tag_close)(input.clone()) {
            return Ok((input, None));
        }

        let (input, (command, _close_tag, whitespace, adjacent_open_tag)) = tuple((
            alt((collapse_whitespace_command, trim_whitespace_command)),
            tag(tag_close),
            opt(whitespace),
            // The group in the `peek()` should match the checks in `tag_start()` after the whitespace check.
            peek(opt(pair(
                tag_open,
                opt(alt((collapse_whitespace_command, trim_whitespace_command))),
            ))),
        ))(input)?;

        if let Some((_open_tag, next_command)) = adjacent_open_tag {
            match (&command, &next_command) {
                (command, Some(next_command)) if command.as_str() != next_command.as_str() => {
                    let mut source = command.clone();
                    source.range.end = next_command.range.end;
                    return Ok((
                        input,
                        Some(Item::CompileError(
                            "Mismatched whitespace adjustment characters".to_owned(),
                            source,
                        )),
                    ));
                }
                (_, _) => (),
            }
        }

        let whitespace = match command.as_str() {
            "_" => whitespace.map(|whitespace| Static(" ", whitespace)),
            "-" => None,
            _ => unreachable!("Only - or _ should be matched"),
        };

        Ok((input, whitespace.map(Item::Whitespace)))
    }
}

pub(crate) fn tag_open(input: Source) -> Res<Source, TagOpen> {
    let (input, output) = alt((
        tag("{{"), // writ
        tag("{%"), // statement
        tag("{#"), // comment
    ))(input)?;

    match output.as_str() {
        "{{" => Ok((input, TagOpen::Writ(output))),
        "{%" => Ok((input, TagOpen::Statement(output))),
        "{#" => Ok((input, TagOpen::Comment(output))),
        _ => panic!("This should never happen"),
    }
}

fn collapse_whitespace_command(input: Source) -> Res<Source, Source> {
    tag("_")(input)
}

fn trim_whitespace_command(input: Source) -> Res<Source, Source> {
    tag("-")(input)
}

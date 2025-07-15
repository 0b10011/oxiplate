use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, opt, peek};
use nom::sequence::pair;
use nom::Parser as _;
use nom_language::error::VerboseError;
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

/// One piece of a template.
#[derive(Debug)]
pub(crate) enum Item<'a> {
    /// A private comment that should be discarded from the final output.
    Comment,

    /// An expression that should be evaluated and output.
    Writ(Writ<'a>),

    /// A statement that should be evaluated.
    Statement(Statement<'a>),

    /// Static text that should be output as-is,
    /// except whitespace that may not be in some contexts.
    Static(Static<'a>, StaticType),

    /// Whitespace that could be collapsed or removed.
    Whitespace(Static<'a>),

    /// A template error encountered during compliation.
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
            TagOpen::Writ(_source) => cut(writ(state)).parse(input)?,
            TagOpen::Statement(_source) => cut(statement(state, is_extending)).parse(input)?,
            TagOpen::Comment(_source) => cut(comment).parse(input)?,
        };

        let mut items = vec![];

        if let Some(leading_whitespace) = leading_whitespace {
            items.push(leading_whitespace);
        }

        items.push(tag);

        if let Some(trailing_whitespace) = trailing_whitespace {
            items.push(trailing_whitespace);
        }

        Ok((input, items))
    }
}

pub(crate) fn tag_start(input: Source) -> Res<Source, (Option<Item>, TagOpen)> {
    let (input, (whitespace, open, command)) = (
        // Whitespace is optional, but tracked because it could be altered by tag.
        opt(whitespace),
        // Check if this is actually a tag; if it's not, that's fine, just return early.
        tag_open,
        // Whitespace control characters are optional.
        opt(alt((collapse_whitespace_command, trim_whitespace_command))),
    )
        .parse(input)?;

    let whitespace = if let Some(command) = command {
        match command.as_str() {
            "_" => {
                if whitespace
                    .as_ref()
                    .is_none_or(|whitespace| whitespace.as_str().is_empty())
                {
                    let mut source = match &open {
                        TagOpen::Writ(source)
                        | TagOpen::Statement(source)
                        | TagOpen::Comment(source) => source.clone(),
                    };
                    source.range.end = command.range.end;
                    return Ok((
                        input,
                        (
                            Some(Item::CompileError(
                                "Whitespace replace character `_` used after non-whitespace. \
                                 Either add whitespace or change how whitespace is handled before \
                                 this tag."
                                    .to_owned(),
                                source,
                            )),
                            open,
                        ),
                    ));
                }

                whitespace.map(|whitespace| Static(" ", whitespace))
            }
            "-" => None,
            _ => unreachable!("Only - or _ should be matched"),
        }
    } else {
        whitespace.map(|whitespace| Static(whitespace.as_str(), whitespace))
    };

    Ok((input, (whitespace.map(Item::Whitespace), open)))
}

pub(crate) fn tag_end<'a>(
    tag_close: &'static str,
) -> impl Fn(Source<'a>) -> Res<Source<'a>, Option<Item<'a>>> + 'a {
    move |input| {
        if let Ok((input, _tag)) = tag::<_, _, VerboseError<_>>(tag_close).parse(input.clone()) {
            return Ok((input, None));
        }

        let (input, (command, close_tag, (matched_whitespace, adjacent_open_tag))) = (
            alt((collapse_whitespace_command, trim_whitespace_command)),
            tag(tag_close),
            peek(pair(
                opt(whitespace),
                // The group below should match the checks in `tag_start()` after the whitespace check.
                opt(pair(
                    tag_open,
                    opt(alt((collapse_whitespace_command, trim_whitespace_command))),
                )),
            )),
        )
            .parse(input)?;

        let mut next_command_is_set = false;
        if let Some((_open_tag, next_command)) = adjacent_open_tag {
            if next_command.is_some() {
                next_command_is_set = true;
            }
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

        let (input, matched_whitespace) = match command.as_str() {
            "_" => {
                if matched_whitespace.is_none_or(|whitespace| whitespace.as_str().is_empty()) {
                    let mut source = command.clone();
                    source.range.end = close_tag.range.end;
                    return Ok((
                        input,
                        Some(Item::CompileError(
                            "Whitespace replace character `_` used before non-whitespace. Either \
                             add whitespace or change how whitespace is handled after this tag."
                                .to_owned(),
                            source,
                        )),
                    ));
                } else if next_command_is_set {
                    (input, None)
                } else {
                    let (input, matched_whitespace) = opt(whitespace).parse(input)?;
                    (
                        input,
                        matched_whitespace.map(|whitespace| Static(" ", whitespace)),
                    )
                }
            }
            "-" => {
                let (input, _matched_whitespace) = opt(whitespace).parse(input)?;
                (input, None)
            }
            _ => unreachable!("Only - or _ should be matched"),
        };

        Ok((input, matched_whitespace.map(Item::Whitespace)))
    }
}

pub(crate) fn tag_open(input: Source) -> Res<Source, TagOpen> {
    let (input, output) = alt((
        tag("{{"), // writ
        tag("{%"), // statement
        tag("{#"), // comment
    ))
    .parse(input)?;

    match output.as_str() {
        "{{" => Ok((input, TagOpen::Writ(output))),
        "{%" => Ok((input, TagOpen::Statement(output))),
        "{#" => Ok((input, TagOpen::Comment(output))),
        _ => panic!("This should never happen"),
    }
}

fn collapse_whitespace_command(input: Source) -> Res<Source, Source> {
    tag("_").parse(input)
}

fn trim_whitespace_command(input: Source) -> Res<Source, Source> {
    tag("-").parse(input)
}

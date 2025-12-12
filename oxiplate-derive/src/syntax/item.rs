use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, opt, peek};
use nom::sequence::pair;
use nom_language::error::VerboseError;
use proc_macro::Diagnostic;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use super::comment::comment;
use super::statement::statement;
use super::r#static::StaticType;
use super::template::whitespace;
use super::writ::writ;
use super::{Res, Statement, Static, Writ};
use crate::syntax::statement::StatementKind;
use crate::{Source, State};

pub(super) enum ItemToken {
    StaticText(TokenStream, usize),
    DynamicText(TokenStream, usize),
    Comment,
    Statement(TokenStream, usize),
}

/// One piece of a template.
#[derive(Debug)]
pub(crate) enum Item<'a> {
    /// A private comment that should be discarded from the final output.
    Comment(Source<'a>),

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
    CompileError {
        message: String,
        error_source: Source<'a>,
        consumed_source: Source<'a>,
    },
}

impl<'a> Item<'a> {
    pub(super) fn source(&self) -> &Source<'a> {
        match self {
            Self::Comment(source)
            | Self::CompileError {
                consumed_source: source,
                ..
            } => source,
            Self::Writ(writ) => writ.source(),
            Self::Statement(statement) => statement.source(),
            Self::Static(text, _static_type) => &text.1,
            Self::Whitespace(text) => &text.1,
        }
    }

    pub(super) fn to_token(&'a self, state: &mut State<'a>) -> ItemToken {
        match self {
            Item::Comment(_source) => ItemToken::Comment,
            Item::Writ(writ) => {
                let (text, estimated_length) = writ.to_token(state);
                state.has_content = &true;
                ItemToken::DynamicText(text, estimated_length)
            }
            Item::Statement(statement) => {
                let (statement_tokens, estimated_length) = match statement.to_tokens(state) {
                    Ok(result) => result,
                    Err(result) => {
                        if let StatementKind::DefaultEscaper(_) = statement.kind {
                            state.failed_to_set_default_escaper_group = &true;
                        }

                        result
                    }
                };
                state.has_content = &true;

                if let StatementKind::DefaultEscaper(default_escaper) = &statement.kind
                    && let Some(default_escaper_group) = state
                        .config
                        .escaper_groups
                        .get(default_escaper.escaper.ident)
                {
                    state.default_escaper_group =
                        Some((default_escaper.escaper.ident, default_escaper_group));
                }

                ItemToken::Statement(quote! { #statement_tokens }, estimated_length)
            }
            Item::Static(text, _static_type) => {
                let (text, estimated_length) = text.to_token();
                state.has_content = &true;
                ItemToken::StaticText(text, estimated_length)
            }
            Item::Whitespace(whitespace) => {
                let (text, estimated_length) = whitespace.to_token();
                ItemToken::StaticText(text, estimated_length)
            }
            Item::CompileError {
                message,
                error_source,
                consumed_source: _,
            } => {
                let span = error_source.span();
                ItemToken::Statement(quote_spanned! {span=> compile_error!(#message); }, 0)
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

impl<'a> TagOpen<'a> {
    fn source(&self) -> &Source<'a> {
        match self {
            TagOpen::Writ(source) | TagOpen::Statement(source) | TagOpen::Comment(source) => source,
        }
    }
}

pub(crate) fn parse_tag(input: Source) -> Res<Source, Vec<Item>> {
    let (input, (leading_whitespace, open, source)) = tag_start(input)?;

    let (input, (tag, trailing_whitespace)) = match open {
        TagOpen::Writ(_source) => cut(writ(source)).parse(input)?,
        TagOpen::Statement(_source) => cut(statement(source)).parse(input)?,
        TagOpen::Comment(_source) => cut(comment(source)).parse(input)?,
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

pub(crate) fn tag_start(input: Source) -> Res<Source, (Option<Item>, TagOpen, Source)> {
    let (input, (whitespace, open, command)) = (
        // Whitespace is optional, but tracked because it could be altered by tag.
        opt(whitespace),
        // Check if this is actually a tag; if it's not, that's fine, just return early.
        tag_open,
        // Whitespace control characters are optional.
        opt(alt((collapse_whitespace_command, trim_whitespace_command))),
    )
        .parse(input)?;

    let (whitespace, removed_whitespace) = if let Some(command) = &command {
        match command.as_str() {
            "_" => {
                if whitespace
                    .as_ref()
                    .is_none_or(|whitespace| whitespace.as_str().is_empty())
                {
                    let source = match &open {
                        TagOpen::Writ(source)
                        | TagOpen::Statement(source)
                        | TagOpen::Comment(source) => source.clone(),
                    }
                    .merge(command, "Command expected after tag open");
                    let mut consumed_source = source.clone();
                    consumed_source.range.end = consumed_source.range.start;
                    let mut error_source = source.clone();
                    error_source.range.end = command.range.end;
                    return Ok((
                        input,
                        (
                            Some(Item::CompileError {
                                message: "Whitespace replace character `_` used after \
                                          non-whitespace. Either add whitespace or change how \
                                          whitespace is handled before this tag."
                                    .to_owned(),
                                error_source,
                                consumed_source,
                            }),
                            open,
                            source,
                        ),
                    ));
                }

                (whitespace.map(|whitespace| Static(" ", whitespace)), None)
            }
            "-" => (None, whitespace),
            _ => unreachable!("Only - or _ should be matched"),
        }
    } else {
        (
            whitespace.map(|whitespace| Static(whitespace.as_str(), whitespace)),
            None,
        )
    };

    let source = if let Some(removed_whitespace) = removed_whitespace {
        removed_whitespace.merge(open.source(), "Open tag expected after whitespace")
    } else {
        open.source().clone()
    }
    .merge_some(
        command.as_ref(),
        "Expected whitespace control character after open tag",
    );

    Ok((input, (whitespace.map(Item::Whitespace), open, source)))
}

#[allow(clippy::too_many_lines)]
pub(crate) fn tag_end<'a>(
    tag_close: &'static str,
) -> impl Fn(Source<'a>) -> Res<Source<'a>, (Option<Item<'a>>, Source<'a>)> + 'a {
    move |input| {
        if let Ok((input, tag)) = tag::<_, _, VerboseError<_>>(tag_close).parse(input.clone()) {
            return Ok((input, (None, tag)));
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

        let source = command.clone().merge(
            &close_tag,
            "Tag close expected after whitespace control character",
        );

        let mut next_command_is_set = false;
        if let Some((_open_tag, next_command)) = adjacent_open_tag {
            if next_command.is_some() {
                next_command_is_set = true;
            }
            match (&command, &next_command) {
                (command, Some(next_command)) if command.as_str() != next_command.as_str() => {
                    let mut consumed_source = source.clone();
                    consumed_source.range.start = consumed_source.range.end;
                    let mut error_source = command.clone();
                    error_source.range.end = next_command.range.end;
                    return Ok((
                        input,
                        (
                            Some(Item::CompileError {
                                message: "Mismatched whitespace adjustment characters".to_owned(),
                                error_source,
                                consumed_source,
                            }),
                            command.clone().merge(
                                &close_tag,
                                "Tag close expected after whitespace control character",
                            ),
                        ),
                    ));
                }
                (_, _) => (),
            }
        }

        let (input, matched_whitespace, removed_whitespace) = match command.as_str() {
            "_" => {
                if matched_whitespace.is_none_or(|whitespace| whitespace.as_str().is_empty()) {
                    let mut consumed_source = source.clone();
                    consumed_source.range.start = consumed_source.range.end;
                    let mut error_source = command.clone();
                    error_source.range.end = close_tag.range.end;
                    return Ok((
                        input,
                        (
                            Some(Item::CompileError {
                                message: "Whitespace replace character `_` used before \
                                          non-whitespace. Either add whitespace or change how \
                                          whitespace is handled after this tag."
                                    .to_owned(),
                                error_source,
                                consumed_source,
                            }),
                            command.clone().merge(
                                &close_tag,
                                "Tag close expected after whitespace control character",
                            ),
                        ),
                    ));
                } else if next_command_is_set {
                    (input, None, None)
                } else {
                    let (input, matched_whitespace) = opt(whitespace).parse(input)?;
                    (
                        input,
                        matched_whitespace.map(|whitespace| Static(" ", whitespace)),
                        None,
                    )
                }
            }
            "-" => {
                let (input, matched_whitespace) = opt(whitespace).parse(input)?;
                (input, None, matched_whitespace)
            }
            _ => unreachable!("Only - or _ should be matched"),
        };

        Ok((
            input,
            (
                matched_whitespace.map(Item::Whitespace),
                source.merge_some(
                    removed_whitespace.as_ref(),
                    "Whitespace expected after close tag",
                ),
            ),
        ))
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
        _ => {
            Diagnostic::spanned(
                output.span().unwrap(),
                proc_macro::Level::Error,
                "Internal Oxiplate error: Unsupported open tag encountered",
            )
            .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Unsupported+open+tag+encountered")
            .help("Include template that caused the issue.")
            .emit();
            unreachable!("Internal Oxiplate error. See previous error for more information.");
        }
    }
}

fn collapse_whitespace_command(input: Source) -> Res<Source, Source> {
    tag("_").parse(input)
}

fn trim_whitespace_command(input: Source) -> Res<Source, Source> {
    tag("-").parse(input)
}

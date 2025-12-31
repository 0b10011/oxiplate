use std::collections::HashSet;

use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, opt, peek};
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
#[allow(clippy::large_enum_variant)]
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

    pub(super) fn to_token<'b: 'a>(&'a self, state: &mut State<'b>) -> ItemToken {
        match self {
            Item::Comment(_source) => ItemToken::Comment,
            Item::Writ(writ) => {
                let (text, estimated_length) = writ.to_token(state);
                state.has_content = true;
                ItemToken::DynamicText(text, estimated_length)
            }
            Item::Statement(statement) => {
                let (statement_tokens, estimated_length) = match statement.to_tokens(state) {
                    Ok(result) => result,
                    Err(result) => {
                        if let StatementKind::DefaultEscaper(_) = statement.kind {
                            state.failed_to_set_default_escaper_group = true;
                        }

                        result
                    }
                };
                state.has_content = true;

                if let StatementKind::DefaultEscaper(default_escaper) = &statement.kind
                    && let Some(default_escaper_group) = state
                        .config
                        .escaper_groups
                        .get(default_escaper.escaper.as_str())
                {
                    state.default_escaper_group = Some((
                        default_escaper.escaper.as_str().to_owned(),
                        default_escaper_group.clone(),
                    ));
                }

                if let StatementKind::Let(statement) = &statement.kind {
                    state
                        .local_variables
                        .add(HashSet::from([statement.variable().to_string()]));
                }

                ItemToken::Statement(quote! { #statement_tokens }, estimated_length)
            }
            Item::Static(text, _static_type) => {
                let (text, estimated_length) = text.to_token();
                state.has_content = true;
                ItemToken::StaticText(text, estimated_length)
            }
            Item::Whitespace(whitespace) => {
                if whitespace.0.is_empty() {
                    ItemToken::Comment
                } else {
                    let (text, estimated_length) = whitespace.to_token();
                    ItemToken::StaticText(text, estimated_length)
                }
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
        opt(alt((
            collapse_whitespace_command,
            trim_whitespace_command,
            #[cfg(feature = "unreachable")]
            tag("$"),
        ))),
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
            _ => {
                Diagnostic::spanned(
                    command.span().unwrap(),
                    proc_macro::Level::Error,
                    "Internal Oxiplate error: Unhandled whitespace command in tag start",
                )
                .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Unhandled+whitespace+command+in+tag+start")
                .help("Include template that caused the issue.")
                .emit();
                unreachable!("Internal Oxiplate error. See previous error for more information.");
            }
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

#[derive(Debug, PartialEq, Eq)]
pub(super) enum WhitespacePreference {
    /// Remove all matched whitespace.
    /// Tags that suggest this: `{{- foo -}}` and `{-}`
    Remove,

    /// Replace all matched whitespace with a single space.
    /// Tags that suggest this: `{{_ foo _}}` and `{_}`
    Replace,

    /// Rely on surrounding tags to make a decision,
    /// otherwise leave the whitespace unchanged.
    /// Tag that suggests this: `{{ foo }}`
    Indifferent,
}

fn parse_next_whitespace_preference_statement(
    input: Source,
) -> Res<Source, (WhitespacePreference, Source, bool)> {
    let (input, (trailing_whitespace, (tag, command))) = peek((
        opt(whitespace),
        // The group below should match the checks in `tag_start()` after the whitespace check.
        (
            tag_open,
            opt(alt((
                collapse_whitespace_command,
                trim_whitespace_command,
                #[cfg(feature = "unreachable")]
                tag("$"),
            ))),
        ),
    ))
    .parse(input)?;

    let next_whitespace_preference = if let Some(command) = &command {
        match command.as_str() {
            "-" => WhitespacePreference::Remove,
            "_" => WhitespacePreference::Replace,
            _ => {
                Diagnostic::spanned(
                    command.span().unwrap(),
                    proc_macro::Level::Error,
                    "Internal Oxiplate error: Unhandled whitespace command in next tag start",
                )
                .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Unhandled+whitespace+command+in+next+tag+start")
                .help("Include template that caused the issue.")
                .emit();
                unreachable!("Internal Oxiplate error. See previous error for more information.");
            }
        }
    } else {
        WhitespacePreference::Indifferent
    };
    Ok((
        input,
        (
            next_whitespace_preference,
            tag.source()
                .clone()
                .merge_some(command.as_ref(), "Command expected after start tag"),
            trailing_whitespace.is_some(),
        ),
    ))
}

fn parse_next_whitespace_preference_adjustment_tag(
    input: Source,
) -> Res<Source, (WhitespacePreference, Source, bool)> {
    let (input, (trailing_whitespace, tag)) = peek((
        opt(whitespace),
        // The group below should match the checks in `adjusted_whitespace()` after the whitespace check.
        alt((
            tag("{_}"),
            tag("{-}"),
            #[cfg(feature = "unreachable")]
            tag("{$}"),
        )),
    ))
    .parse(input)?;

    let next_whitespace_preference = match tag.as_str() {
        "{-}" => WhitespacePreference::Remove,
        "{_}" => WhitespacePreference::Replace,
        _ => {
            Diagnostic::spanned(
                tag.span().unwrap(),
                proc_macro::Level::Error,
                "Internal Oxiplate error: Unhandled next whitespace adjustment tag",
            )
            .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Unhandled+next+whitespace+adjustment+tag")
            .help("Include template that caused the issue.")
            .emit();
            unreachable!("Internal Oxiplate error. See previous error for more information.");
        }
    };

    Ok((
        input,
        (
            next_whitespace_preference,
            tag,
            trailing_whitespace.is_some(),
        ),
    ))
}

pub(super) fn parse_trailing_whitespace<'a>(
    end_tag: Source<'a>,
    whitespace_preference: WhitespacePreference,
    allow_skipping_replacement: bool,
) -> impl Fn(Source<'a>) -> Res<Source<'a>, Option<Item<'a>>> + 'a {
    move |input| {
        let (input, next_tag_and_preference) = opt(alt((
            parse_next_whitespace_preference_statement,
            parse_next_whitespace_preference_adjustment_tag,
        )))
        .parse(input)?;

        if let Some((next_whitespace_preference, next_tag_with_command, followed_by_whitespace)) =
            next_tag_and_preference
        {
            match (&whitespace_preference, next_whitespace_preference) {
                (WhitespacePreference::Remove, WhitespacePreference::Replace)
                | (WhitespacePreference::Replace, WhitespacePreference::Remove) => {
                    let mut consumed_source = end_tag.clone();
                    consumed_source.range.start = consumed_source.range.end;
                    let mut error_source = next_tag_with_command;
                    error_source.range.start = end_tag.range.start;
                    return Ok((
                        input,
                        Some(Item::CompileError {
                            message: "Mismatched whitespace adjustment characters".to_owned(),
                            error_source,
                            consumed_source,
                        }),
                    ));
                }
                (_, WhitespacePreference::Remove | WhitespacePreference::Replace) => {
                    // Whitesplace adjustment will be handled by the next tag,
                    // but empty whitespace needs to be returned
                    // so the caller knows not to return an error.
                    if followed_by_whitespace {
                        let mut source = end_tag.clone();
                        source.range.start = source.range.end;

                        return Ok((input, Some(Item::Whitespace(Static("", source)))));
                    }
                }
                _ => (),
            }
        }

        match &whitespace_preference {
            WhitespacePreference::Replace => {
                let (input, trailing_whitespace) = opt(whitespace).parse(input)?;

                let item = if let Some(trailing_whitespace) = trailing_whitespace {
                    Item::Whitespace(Static(" ", trailing_whitespace))
                } else if allow_skipping_replacement {
                    return Ok((input, None));
                } else {
                    let mut consumed_source = end_tag.clone();
                    consumed_source.range.start = consumed_source.range.end;
                    let error_source = end_tag.clone();
                    Item::CompileError {
                        message: "Whitespace replace character `_` used before non-whitespace. \
                                  Either add whitespace or change how whitespace is handled after \
                                  this tag."
                            .to_owned(),
                        error_source,
                        consumed_source,
                    }
                };

                Ok((input, Some(item)))
            }
            WhitespacePreference::Remove => {
                let (input, trailing_whitespace) = opt(whitespace).parse(input)?;
                Ok((
                    input,
                    trailing_whitespace.map(|whitespace| Item::Whitespace(Static("", whitespace))),
                ))
            }
            WhitespacePreference::Indifferent => Ok((input, None)),
        }
    }
}

pub(crate) fn tag_end<'a>(
    tag_close: &'static str,
) -> impl Fn(Source<'a>) -> Res<Source<'a>, (Option<Item<'a>>, Source<'a>)> + 'a {
    move |input| {
        let (input, (command, close_tag)) = (
            opt(alt((
                collapse_whitespace_command,
                trim_whitespace_command,
                #[cfg(feature = "unreachable")]
                tag("$"),
            ))),
            tag(tag_close),
        )
            .parse(input)?;

        let source = command.clone().map_or(close_tag.clone(), |source| {
            source.merge(
                &close_tag,
                "Tag close expected after whitespace control character",
            )
        });

        let whitespace_preference = match command.clone().map(|source| source.as_str()) {
            None => WhitespacePreference::Indifferent,
            Some("-") => WhitespacePreference::Remove,
            Some("_") => WhitespacePreference::Replace,
            Some(_) => {
                Diagnostic::spanned(
                    command.expect("Command is already matched as `Some(_)`").span().unwrap(),
                    proc_macro::Level::Error,
                    "Internal Oxiplate error: Unhandled whitespace command in tag end",
                )
                .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Unhandled+whitespace+command+in+tag+end")
                .help("Include template that caused the issue.")
                .emit();
                unreachable!("Internal Oxiplate error. See previous error for more information.");
            }
        };

        let (input, trailing_whitespace) =
            parse_trailing_whitespace(source.clone(), whitespace_preference, false).parse(input)?;

        Ok((input, (trailing_whitespace, source)))
    }
}

pub(crate) fn tag_open(input: Source) -> Res<Source, TagOpen> {
    let (input, output) = alt((
        tag("{{"), // writ
        tag("{%"), // statement
        tag("{#"), // comment
        #[cfg(feature = "unreachable")]
        tag("{&"),
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

pub(super) fn collapse_whitespace_command(input: Source) -> Res<Source, Source> {
    tag("_").parse(input)
}

pub(super) fn trim_whitespace_command(input: Source) -> Res<Source, Source> {
    tag("-").parse(input)
}

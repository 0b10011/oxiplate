use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use super::comment::comment;
use super::statement::statement;
use super::r#static::StaticType;
use super::writ::writ;
use super::{Res, Statement, Static, Writ};
use crate::syntax::Error;
use crate::syntax::parser::{Parser as _, cut, opt, take};
use crate::syntax::statement::StatementKind;
use crate::tokenizer::{TagKind, TokenKind, TokenSlice, WhitespacePreference};
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

                if let StatementKind::DefaultEscaper(default_escaper) = &statement.kind {
                    if let Some(default_escaper_group) = state
                        .config
                        .escaper_groups
                        .get(default_escaper.escaper.as_str())
                    {
                        state.default_escaper_group = Some((
                            default_escaper.escaper.as_str().to_owned(),
                            default_escaper_group.clone(),
                        ));
                    }
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
                let span = error_source.span_token();
                ItemToken::Statement(quote_spanned! {span=> compile_error!(#message); }, 0)
            }
        }
    }
}

#[derive(Debug)]
pub struct TagOpen<'a> {
    source: Source<'a>,
    kind: TagKind,
    whitespace_preference: WhitespacePreference,
}

impl<'a> TagOpen<'a> {
    fn source(&self) -> &Source<'a> {
        &self.source
    }
}

pub(crate) fn parse_tag(tokens: TokenSlice) -> Res<Vec<Item>> {
    let (tokens, (leading_whitespace, open, source)) = tag_start.parse(tokens)?;

    let (tokens, (tag, trailing_whitespace)) = match open.kind {
        TagKind::Writ => cut("Expected a writ expression", writ(source)).parse(tokens)?,
        TagKind::Statement => cut("Expected a statement", statement(source)).parse(tokens)?,
        TagKind::Comment => cut("Expected a comment", comment(source)).parse(tokens)?,
    };

    let mut items = vec![];

    if let Some(leading_whitespace) = leading_whitespace {
        items.push(leading_whitespace);
    }

    items.push(tag);

    if let Some(trailing_whitespace) = trailing_whitespace {
        items.push(trailing_whitespace);
    }

    Ok((tokens, items))
}

pub(crate) fn tag_start(tokens: TokenSlice) -> Res<(Option<Item>, TagOpen, Source)> {
    let (tokens, (leading_whitespace, open)) =
        (opt(take(TokenKind::StaticWhitespace)), tag_open).parse(tokens)?;

    let (whitespace, removed_whitespace) = match open.whitespace_preference {
        WhitespacePreference::Replace => {
            if leading_whitespace
                .as_ref()
                .is_none_or(|whitespace| whitespace.source().as_str().is_empty())
            {
                let source = open.source().clone();
                let consumed_source = source.with_collapsed_to_start_full();
                let error_source = source.clone();
                return Ok((
                    tokens,
                    (
                        Some(Item::CompileError {
                            message: "Whitespace replace character `_` used after non-whitespace. \
                                      Either add whitespace or change how whitespace is handled \
                                      before this tag."
                                .to_owned(),
                            error_source,
                            consumed_source,
                        }),
                        open,
                        source,
                    ),
                ));
            }

            (
                leading_whitespace.map(|whitespace| Static(" ", whitespace.source().clone())),
                None,
            )
        }
        WhitespacePreference::Remove => (None, leading_whitespace),
        WhitespacePreference::Indifferent => (
            leading_whitespace.map(|whitespace| {
                Static(whitespace.source().as_str(), whitespace.source().clone())
            }),
            None,
        ),
    };

    let source = open.source().clone().append_to_some(
        removed_whitespace.map(|whitespace| whitespace.source().clone()),
        "Open tag expected after whitespace",
    );

    Ok((tokens, (whitespace.map(Item::Whitespace), open, source)))
}

pub(super) fn parse_trailing_whitespace<'a>(
    end_tag: &'a Source<'a>,
    whitespace_preference: &'a WhitespacePreference,
    allow_skipping_replacement: bool,
) -> impl Fn(TokenSlice<'a>) -> Res<'a, Option<Item<'a>>> + 'a {
    move |tokens| {
        // Clone tokens to prevent taking anything until later
        let peek_tokens = tokens.clone();

        let (peek_tokens, trailing_whitespace) =
            opt(take(TokenKind::StaticWhitespace)).parse(peek_tokens)?;

        let next_token = match peek_tokens.take() {
            Ok((_peek_tokens, next_token)) => Some(next_token),
            Err(err) => {
                if err.is_eof() {
                    None
                } else {
                    return Err(err.into());
                }
            }
        };

        if let Some(next_token) = next_token {
            let next_whitespace_preference = match next_token.kind() {
                TokenKind::TagStart {
                    whitespace_preference,
                    ..
                }
                | TokenKind::WhitespaceAdjustmentTag {
                    whitespace_preference,
                } => whitespace_preference,
                _ => &WhitespacePreference::Indifferent,
            };

            match (whitespace_preference, next_whitespace_preference) {
                (WhitespacePreference::Remove, WhitespacePreference::Replace)
                | (WhitespacePreference::Replace, WhitespacePreference::Remove) => {
                    let consumed_source = end_tag.with_collapsed_to_end();
                    let error_source = next_token.source().with_start(end_tag);
                    return Ok((
                        tokens,
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
                    if trailing_whitespace.is_some() {
                        let source = end_tag.with_collapsed_to_end();

                        return Ok((tokens, Some(Item::Whitespace(Static("", source)))));
                    }
                }
                _ => (),
            }
        }

        match whitespace_preference {
            WhitespacePreference::Replace => {
                let (tokens, trailing_whitespace) =
                    opt(take(TokenKind::StaticWhitespace)).parse(tokens)?;

                let item = if let Some(trailing_whitespace) = trailing_whitespace {
                    Item::Whitespace(Static(" ", trailing_whitespace.source().clone()))
                } else if allow_skipping_replacement {
                    return Ok((tokens, None));
                } else {
                    let consumed_source = end_tag.with_collapsed_to_end();
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

                Ok((tokens, Some(item)))
            }
            WhitespacePreference::Remove => {
                let (tokens, trailing_whitespace) =
                    opt(take(TokenKind::StaticWhitespace)).parse(tokens)?;
                Ok((
                    tokens,
                    trailing_whitespace.map(|whitespace| {
                        Item::Whitespace(Static("", whitespace.source().clone()))
                    }),
                ))
            }
            WhitespacePreference::Indifferent => Ok((tokens, None)),
        }
    }
}

pub(crate) fn tag_end<'a>(
    expected_kind: TagKind,
) -> impl Fn(TokenSlice<'a>) -> Res<'a, (Option<Item<'a>>, Source<'a>)> {
    move |tokens| {
        let (tokens, token) = tokens.take()?;

        let TokenKind::TagEnd {
            kind,
            whitespace_preference,
        } = token.kind()
        else {
            return Err(Error::Recoverable {
                message: format!("Expected `TagEnd`, found `{:?}`", token.kind()),
                source: token.source().clone(),
                previous_error: None,
                is_eof: false,
            });
        };

        if *kind != expected_kind {
            return Err(Error::Unrecoverable {
                message: format!("Expected `{expected_kind:?}`, found `{kind:?}`"),
                source: token.source().clone(),
                previous_error: None,
                is_eof: false,
            });
        }

        let (tokens, trailing_whitespace) =
            parse_trailing_whitespace(token.source(), whitespace_preference, false)
                .parse(tokens)?;

        Ok((tokens, (trailing_whitespace, token.source().clone())))
    }
}

pub(crate) fn tag_open(tokens: TokenSlice) -> Res<TagOpen> {
    let (tokens, token) = tokens.take()?;

    let TokenKind::TagStart {
        kind,
        whitespace_preference,
    } = token.kind()
    else {
        return Err(Error::Recoverable {
            message: "Not `TagStart` token".to_string(),
            source: token.source().clone(),
            previous_error: None,
            is_eof: false,
        });
    };

    Ok((
        tokens,
        TagOpen {
            source: token.source().clone(),
            kind: kind.clone(),
            whitespace_preference: whitespace_preference.clone(),
        },
    ))
}

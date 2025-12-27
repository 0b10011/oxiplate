mod block;
mod escaper;
mod extends;
mod r#for;
mod helpers;
mod r#if;
mod include;
mod r#match;

use block::Block;
use extends::Extends;
use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::combinator::{cut, into, opt};
use nom::error::context;
use proc_macro2::TokenStream;
use quote::quote_spanned;

pub(crate) use self::escaper::DefaultEscaper;
use self::r#for::For;
use self::r#if::{ElseIf, If};
use self::include::Include;
use super::r#static::StaticType;
use super::{Item, Res};
use crate::syntax::item::tag_end;
use crate::syntax::statement::r#for::{Break, Continue};
use crate::syntax::statement::r#match::{Case, Match};
use crate::syntax::template::{is_whitespace, parse_item, whitespace};
use crate::{Source, State};

#[derive(Debug)]
pub(crate) struct Statement<'a> {
    source: Source<'a>,
    pub(crate) kind: StatementKind<'a>,
}

#[derive(Debug)]
pub(crate) enum StatementKind<'a> {
    DefaultEscaper(DefaultEscaper<'a>),

    Extends(Extends<'a>),
    Block(Block<'a>),
    Parent,
    EndBlock,

    Include(Include<'a>),

    If(If<'a>),
    ElseIf(ElseIf<'a>),
    Else,
    EndIf,

    For(For<'a>),
    Continue(Continue<'a>),
    Break(Break<'a>),
    EndFor,

    Match(Match<'a>),
    Case(Case<'a>),
    EndMatch,
}

impl<'a> Statement<'a> {
    pub(super) fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub(super) fn wrap_source(&mut self, prefix: Source<'a>, suffix: &Source<'a>) {
        self.source = prefix
            .merge(&self.source, "Prefix should be followed by source")
            .merge(suffix, "Suffix should follow source");
    }

    pub fn is_ended(&self, is_eof: bool) -> bool {
        #[allow(clippy::enum_glob_use)]
        use StatementKind::*;
        match &self.kind {
            Extends(_) => is_eof,
            Block(statement) => statement.is_ended,
            If(statement) => statement.is_ended,
            For(statement) => statement.is_ended,
            Match(statement) => statement.is_ended(),
            DefaultEscaper(_) | Parent | EndBlock | Include(_) | ElseIf(_) | Else | EndIf
            | Continue(_) | Break(_) | EndFor | Case(_) | EndMatch => true,
        }
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        #[allow(clippy::enum_glob_use)]
        use StatementKind::*;

        self.source = self
            .source
            .clone()
            .merge(item.source(), "Item should follow previous item");

        match &mut self.kind {
            Extends(statement) => {
                statement.add_item(item);
            }
            Block(statement) => {
                statement.add_item(item);
            }
            If(statement) => {
                statement.add_item(item);
            }
            For(statement) => {
                statement.add_item(item);
            }
            Match(statement) => {
                statement.add_item(item);
            }
            DefaultEscaper(_) | Parent | EndBlock | Include(_) | ElseIf(_) | Else | EndIf
            | Continue(_) | Break(_) | EndFor | Case(_) | EndMatch => {
                unreachable!("add_item() should not be called for this kind of statement")
            }
        }
    }

    pub(crate) fn to_tokens(
        &self,
        state: &State,
    ) -> Result<(TokenStream, usize), (TokenStream, usize)> {
        macro_rules! unexpected {
            ($tag:literal) => {{
                let span = self.source.span();
                Err((
                    quote_spanned! {span=> compile_error!(concat!("Unexpected '", $tag, "' statement")); },
                    0,
                ))
            }};
        }

        match &self.kind {
            StatementKind::DefaultEscaper(default_escaper) => {
                default_escaper.to_tokens(state, &self.source)
            }
            StatementKind::Extends(statement) => {
                if *state.has_content {
                    let span = self.source.span();
                    Err((
                        quote_spanned! {span=> compile_error!("Unexpected 'extends' statement after content already present in template"); },
                        0,
                    ))
                } else {
                    Ok(statement.to_tokens(state))
                }
            }
            StatementKind::Block(block) => Ok(block.to_tokens(state)),
            StatementKind::Parent => unexpected!("parent"),
            StatementKind::EndBlock => unexpected!("endblock"),
            StatementKind::Include(statement) => Ok(statement.to_tokens()),
            StatementKind::If(statement) => Ok(statement.to_tokens(state)),
            StatementKind::ElseIf(_) => unexpected!("elseif"),
            StatementKind::Else => unexpected!("else"),
            StatementKind::EndIf => unexpected!("endif"),
            StatementKind::For(statement) => Ok(statement.to_tokens(state)),
            StatementKind::Continue(statement) => Ok(statement.to_tokens()),
            StatementKind::Break(statement) => Ok(statement.to_tokens()),
            StatementKind::EndFor => unexpected!("endfor"),
            StatementKind::Match(statement) => Ok(statement.to_tokens(state)),
            StatementKind::Case(_) => unexpected!("case"),
            StatementKind::EndMatch => unexpected!("endmatch"),
        }
    }
}

impl<'a> From<Statement<'a>> for Item<'a> {
    fn from(statement: Statement<'a>) -> Self {
        Item::Statement(statement)
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn statement<'a>(
    open_tag_source: Source<'a>,
) -> impl Fn(Source<'a>) -> Res<Source<'a>, (Item<'a>, Option<Item<'a>>)> {
    move |input| {
        // Ignore any leading inner whitespace
        let (input, leading_whitespace) = take_while(is_whitespace).parse(input)?;

        // Parse statements
        let (input, mut statement) = context(
            "Expected one of: block, endblock, if, elseif, else, endif, for, continue, break, \
             endfor, match, case, endmatch",
            cut(alt((
                escaper::parse_default_escaper_group,
                extends::parse_extends,
                include::parse_include,
                block::parse_block,
                block::parse_parent,
                block::parse_endblock,
                r#if::parse_if,
                r#if::parse_elseif,
                r#if::parse_else,
                r#if::parse_endif,
                r#for::parse_for,
                into(Continue::parse),
                into(Break::parse),
                r#for::parse_endfor,
                r#match::Match::parse,
                r#match::Case::parse,
                r#match::Match::parse_end,
            ))),
        )
        .parse(input)?;

        // Parse the closing tag and any trailing whitespace
        let (mut input, (whitespace, (mut trailing_whitespace, close_tag))) = (
            opt(whitespace),
            context(r#""%}" expected"#, cut(tag_end("%}"))),
        )
            .parse(input)?;

        statement.wrap_source(
            open_tag_source
                .clone()
                .merge(&leading_whitespace, "Whitespace expected after open tag"),
            &whitespace.map_or(close_tag.clone(), |source| {
                source.merge(&close_tag, "Close tag expected after whitespace")
            }),
        );

        if !statement.is_ended(input.as_str().is_empty()) {
            // Append trailing whitespace
            if let Some(trailing_whitespace) = trailing_whitespace {
                statement.add_item(trailing_whitespace);
            }
            trailing_whitespace = None;

            loop {
                {
                    let is_eof = input.as_str().is_empty();
                    if is_eof {
                        macro_rules! context_message {
                            ($lit:literal) => {
                                concat!(
                                    r#"""#,
                                    $lit,
                                    r#"" statement is never closed (unexpected end of template)"#
                                )
                            };
                        }
                        let context_message = match statement.kind {
                            StatementKind::Block(_) => context_message!("block"),
                            StatementKind::If(_) => context_message!("if"),
                            StatementKind::For(_) => context_message!("for"),
                            StatementKind::Match(_) => context_message!("match"),
                            StatementKind::DefaultEscaper(_)
                            | StatementKind::Extends(_)
                            | StatementKind::Parent
                            | StatementKind::EndBlock
                            | StatementKind::Include(_)
                            | StatementKind::ElseIf(_)
                            | StatementKind::Else
                            | StatementKind::EndIf
                            | StatementKind::Continue(_)
                            | StatementKind::Break(_)
                            | StatementKind::EndFor
                            | StatementKind::Case(_)
                            | StatementKind::EndMatch => unreachable!(
                                "These blocks should never fail to be closed because of EOF"
                            ),
                        };
                        statement.add_item(Item::CompileError {
                            message: context_message.to_string(),
                            error_source: input.clone(),
                            consumed_source: input.clone(),
                        });
                        return Ok((input, (statement.into(), trailing_whitespace)));
                    }
                }

                let (new_input, items) =
                    context("Failed to parse contents of statement", cut(parse_item))
                        .parse(input)?;
                input = new_input;
                for item in items {
                    if statement.is_ended(false) {
                        match item {
                            Item::Whitespace(_) | Item::CompileError { .. } => {
                                trailing_whitespace = Some(item);
                                continue;
                            }
                            _ => statement.add_item(Item::CompileError {
                                message: "Internal Oxiplate error. Unexpected item found immediately following end tag. Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Unexpected+item+found+immediately+following+end+tag".to_string(),
                                error_source: item.source().clone(),
                                consumed_source: item.source().clone(),
                            }),
                        }
                    }

                    statement.add_item(item);
                }

                let is_eof = input.as_str().is_empty();
                #[cfg(not(feature = "unreachable"))]
                if statement.is_ended(is_eof) {
                    break;
                }
                #[cfg(feature = "unreachable")]
                if is_eof {
                    break;
                }
            }
        }

        // Return the statement and trailing whitespace
        Ok((input, (statement.into(), trailing_whitespace)))
    }
}

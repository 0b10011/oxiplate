mod block;
mod escaper;
mod extends;
mod r#for;
mod helpers;
mod r#if;
mod include;
mod r#let;
mod r#match;

use quote::quote_spanned;

pub(crate) use self::escaper::DefaultEscaper;
use super::r#static::StaticType;
use super::{Item, Res};
use crate::syntax::Error;
use crate::syntax::item::tag_end;
use crate::syntax::parser::{Parser as _, alt, cut, into};
use crate::syntax::statement::r#let::Let;
use crate::syntax::template::parse_item;
use crate::tokenizer::parser::{TagKind, TokenSlice};
use crate::{BuiltTokens, Source, State};

#[derive(Debug)]
pub(crate) struct Statement<'a> {
    source: Source<'a>,
    pub(crate) kind: StatementKind<'a>,
}

#[derive(Debug)]
pub(crate) enum StatementKind<'a> {
    DefaultEscaper(DefaultEscaper<'a>),

    Extends(extends::Extends<'a>),
    Block(block::Block<'a>),
    Parent,
    EndBlock,

    Include(include::Include<'a>),

    If(r#if::If<'a>),
    ElseIf(r#if::ElseIf<'a>),
    Else,
    EndIf,

    For(r#for::For<'a>),
    Continue(r#for::Continue<'a>),
    Break(r#for::Break<'a>),
    EndFor,

    Match(r#match::Match<'a>),
    Case(r#match::Case<'a>),
    EndMatch,

    Let(Let<'a>),
}

impl StatementKind<'_> {
    /// Whether this kind of statement is expected to appear in most statements.
    /// Useful for improving error messages
    /// when statements that are specially handled in some kinds of statements
    /// appear in other kinds of statements.
    pub fn expected_in_statements(&self) -> bool {
        match self {
            Self::Extends(_)
            | Self::Parent
            | Self::EndBlock
            | Self::DefaultEscaper(_)
            | Self::ElseIf(_)
            | Self::Else
            | Self::EndIf
            | Self::EndFor
            | Self::Case(_)
            | Self::EndMatch => false,

            Self::Block(_)
            | Self::Include(_)
            | Self::If(_)
            | Self::For(_)
            | Self::Continue(_)
            | Self::Break(_)
            | Self::Match(_)
            | Self::Let(_) => true,
        }
    }
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
            | Continue(_) | Break(_) | EndFor | Case(_) | EndMatch | Let(_) => true,
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
            Extends(statement) => statement.add_item(item),
            Block(statement) => statement.add_item(item),
            If(statement) => statement.add_item(item),
            For(statement) => statement.add_item(item),
            Match(statement) => statement.add_item(item),
            DefaultEscaper(_) | Parent | EndBlock | Include(_) | ElseIf(_) | Else | EndIf
            | Continue(_) | Break(_) | EndFor | Case(_) | EndMatch | Let(_) => {
                unreachable!("add_item() should not be called for this kind of statement")
            }
        }
    }

    pub(crate) fn to_tokens<'b: 'a>(
        &self,
        state: &mut State<'b>,
    ) -> Result<BuiltTokens, BuiltTokens> {
        macro_rules! unexpected {
            ($tag:literal) => {{
                let span = self.source.span_token();
                Err((
                    quote_spanned! {span=> compile_error!(concat!("Unexpected '", $tag, "' statement")); },
                    0,
                ))
            }};
        }

        state.local_variables.push_stack();

        let tokens = match &self.kind {
            StatementKind::DefaultEscaper(default_escaper) => {
                default_escaper.to_tokens(state, &self.source)
            }
            StatementKind::Extends(statement) => {
                if state.has_content {
                    let span = self.source.span_token();
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
            StatementKind::Let(statement) => Ok(statement.to_tokens(state)),
        };

        state.local_variables.pop_stack();

        tokens
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
) -> impl Fn(TokenSlice<'a>) -> Res<'a, (Item<'a>, Option<Item<'a>>)> {
    move |tokens| {
        // Parse statements
        let (tokens, mut statement): (TokenSlice<'a>, Statement<'a>) = cut(
            "Expected one of: default_escaper_group, replace_escaper_group, extends, block, \
             endblock, include, if, elseif, else, endif, for, continue, break, endfor, match, \
             case, endmatch, let",
            alt((
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
                into(r#for::Continue::parse),
                into(r#for::Break::parse),
                r#for::parse_endfor,
                r#match::Match::parse,
                r#match::Case::parse,
                r#match::Match::parse_end,
                into(Let::parse),
            )),
        )
        .parse(tokens)?;

        // Parse the closing tag and any trailing whitespace
        let (tokens, (mut trailing_whitespace, close_tag)) =
            cut(r#""%}" expected"#, tag_end(TagKind::Statement)).parse(tokens)?;

        statement.wrap_source(open_tag_source.clone(), &close_tag);

        let mut tokens = tokens;
        if !statement.is_ended(tokens.is_empty()) {
            // Append trailing whitespace
            if let Some(trailing_whitespace) = trailing_whitespace {
                statement.add_item(trailing_whitespace);
            }
            trailing_whitespace = None;

            loop {
                if tokens.is_empty() {
                    return Ok(eof(tokens, statement, trailing_whitespace));
                }

                let (new_tokens, items) = match statement_item(tokens)? {
                    (new_tokens, Some(items)) => (new_tokens, items),
                    (tokens, None) => {
                        return Ok(eof(tokens, statement, trailing_whitespace));
                    }
                };
                tokens = new_tokens;

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

                let is_eof = tokens.is_empty();
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
        Ok((tokens, (statement.into(), trailing_whitespace)))
    }
}

fn statement_item(tokens: TokenSlice) -> Result<(TokenSlice, Option<Vec<Item>>), Error> {
    let eof = tokens.eof();
    match cut("Failed to parse contents of statement", parse_item).parse(tokens) {
        Ok((tokens, items)) => Ok((tokens, Some(items))),
        Err(err) => {
            if err.is_eof() {
                Ok((TokenSlice::new(&[], eof), None))
            } else {
                Err(err)
            }
        }
    }
}

fn eof<'a>(
    tokens: TokenSlice<'a>,
    mut statement: Statement<'a>,
    trailing_whitespace: Option<Item<'a>>,
) -> (TokenSlice<'a>, (Item<'a>, Option<Item<'a>>)) {
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
        | StatementKind::EndMatch
        | StatementKind::Let(_) => {
            unreachable!("These blocks should never fail to be closed because of EOF")
        }
    };
    statement.add_item(Item::CompileError {
        message: context_message.to_string(),
        error_source: tokens.eof().source().clone(),
        consumed_source: tokens.eof().source().clone(),
    });

    (tokens, (statement.into(), trailing_whitespace))
}

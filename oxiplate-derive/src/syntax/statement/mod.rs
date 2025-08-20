use std::collections::HashSet;

mod block;
mod extends;
mod r#for;
mod r#if;
mod include;

use block::Block;
use extends::Extends;
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::combinator::{cut, fail};
use nom::error::context;
use nom::sequence::preceded;
use nom::Parser as _;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, TokenStreamExt};

use self::r#for::For;
use self::r#if::{ElseIf, If};
use super::r#static::StaticType;
use super::{Item, Res};
use crate::syntax::item::tag_end;
use crate::syntax::statement::include::Include;
use crate::syntax::template::{is_whitespace, parse_item};
use crate::{Source, State};

#[derive(Debug)]
pub(crate) struct Statement<'a> {
    source: Source<'a>,
    pub(crate) kind: StatementKind<'a>,
}

#[derive(Debug)]
pub(crate) enum StatementKind<'a> {
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
    EndFor,
}

impl<'a> Statement<'a> {
    pub fn is_ended(&self, is_eof: bool) -> bool {
        #[allow(clippy::enum_glob_use)]
        use StatementKind::*;
        match &self.kind {
            Extends(_) => is_eof,
            Block(statement) => statement.is_ended,
            If(statement) => statement.is_ended,
            For(statement) => statement.is_ended,
            Parent | EndBlock | Include(_) | ElseIf(_) | Else | EndIf | EndFor => true,
        }
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        #[allow(clippy::enum_glob_use)]
        use StatementKind::*;
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
            Parent | EndBlock | Include(_) | ElseIf(_) | Else | EndIf | EndFor => {
                unreachable!("add_item() should not be called for this kind of statement")
            }
        }
    }

    pub fn get_active_variables(&self) -> HashSet<&'a str> {
        match &self.kind {
            StatementKind::For(statement) => statement.get_active_variables(),
            StatementKind::If(statement) => statement.get_active_variables(),
            _ => HashSet::new(),
        }
    }

    pub(crate) fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let mut tokens = TokenStream::new();
        let mut estimated_length = 0;

        tokens.append_all(match &self.kind {
            StatementKind::Extends(statement) => {
                if *state.has_content {
                    let span = self.source.span();
                    quote_spanned! {span=> compile_error!("Unexpected 'extends' statement after content already present in template"); }
                } else {
                    let (statement, statement_length) = statement.to_tokens(state);
                    estimated_length += statement_length;
                    statement
                }
            }
            StatementKind::Block(block) => {
                let mut local_variables = self.get_active_variables();
                for value in state.local_variables {
                    local_variables.insert(value);
                }
                let state = &State {
                    local_variables: &local_variables,
                    ..*state
                };
                let (block, block_length) = block.to_tokens(state);
                estimated_length += block_length;
                quote! { #block }
            }
            StatementKind::Parent => {
                let span = self.source.span();
                quote_spanned! {span=> compile_error!("Unexpected 'parent' statement"); }
            }
            StatementKind::EndBlock => {
                let span = self.source.span();
                quote_spanned! {span=> compile_error!("Unexpected 'endblock' statement"); }
            }

            StatementKind::Include(statement) => {
                let (statement, statement_length) = statement.to_tokens();
                estimated_length += statement_length;
                quote! { #statement }
            }

            StatementKind::If(statement) => {
                let (statement, statement_length) = statement.to_tokens(state);
                estimated_length += statement_length;
                quote! { #statement }
            }
            StatementKind::ElseIf(_) => {
                let span = self.source.span();
                quote_spanned! {span=> compile_error!("Unexpected 'elseif' statement"); }
            }
            StatementKind::Else => {
                let span = self.source.span();
                quote_spanned! {span=> compile_error!("Unexpected 'else' statement"); }
            }
            StatementKind::EndIf => {
                let span = self.source.span();
                quote_spanned! {span=> compile_error!("Unexpected 'endif' statement"); }
            }

            StatementKind::For(statement) => {
                let (statement, statement_length) = statement.to_tokens(state);
                estimated_length += statement_length;
                quote! { #statement }
            }
            StatementKind::EndFor => {
                let span = self.source.span();
                quote_spanned! {span=> compile_error!("Unexpected 'endfor' statement"); }
            }
        });

        (tokens, estimated_length)
    }
}

impl<'a> From<Statement<'a>> for Item<'a> {
    fn from(statement: Statement<'a>) -> Self {
        Item::Statement(statement)
    }
}

pub(super) fn statement(input: Source) -> Res<Source, (Item, Option<Item>)> {
    // Ignore any leading inner whitespace
    let (input, _) = take_while(is_whitespace).parse(input)?;

    // Parse statements
    let (input, mut statement) = context(
        "Expected one of: block, endblock, if, elseif, else, endif, for, endfor",
        cut(alt((
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
            r#for::parse_endfor,
        ))),
    )
    .parse(input)?;

    // Parse the closing tag and any trailing whitespace
    let (mut input, mut trailing_whitespace) = preceded(
        take_while(is_whitespace),
        context(r#""%}" expected"#, cut(tag_end("%}"))),
    )
    .parse(input)?;

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
                        StatementKind::Extends(_)
                        | StatementKind::Parent
                        | StatementKind::EndBlock
                        | StatementKind::Include(_)
                        | StatementKind::ElseIf(_)
                        | StatementKind::Else
                        | StatementKind::EndIf
                        | StatementKind::EndFor => unreachable!(
                            "These blocks should never fail to be closed because of EOF"
                        ),
                    };
                    return context(context_message, fail()).parse(input);
                }
            }

            let (new_input, items) =
                context("Failed to parse contents of statement", cut(parse_item)).parse(input)?;
            input = new_input;
            for item in items {
                if statement.is_ended(false) {
                    match item {
                        Item::Whitespace(_) | Item::CompileError(_, _) => {
                            trailing_whitespace = Some(item);
                            continue;
                        }
                        _ => (),
                    }
                }

                statement.add_item(item);
            }

            let is_eof = input.as_str().is_empty();
            if statement.is_ended(is_eof) {
                break;
            }
        }
    }

    // Return the statement and trailing whitespace
    Ok((input, (statement.into(), trailing_whitespace)))
}

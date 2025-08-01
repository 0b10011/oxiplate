use std::collections::HashSet;

mod block;
mod extends;
mod r#for;
mod r#if;

use block::Block;
use extends::Extends;
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::combinator::{cut, fail};
use nom::error::context;
use nom::sequence::preceded;
use nom::Parser as _;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

use self::r#for::For;
use self::r#if::{ElseIf, If};
use super::r#static::StaticType;
use super::{Item, Res};
use crate::syntax::item::tag_end;
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
    If(If<'a>),
    ElseIf(ElseIf<'a>),
    Else,
    EndIf,
    For(For<'a>),
    EndFor,
}

impl<'a> Statement<'a> {
    pub fn is_ended(&self, is_eof: bool) -> bool {
        match &self.kind {
            StatementKind::Extends(_) => is_eof,
            StatementKind::Block(statement) => statement.is_ended,
            StatementKind::If(statement) => statement.is_ended,
            StatementKind::For(statement) => statement.is_ended,
            _ => true, /* unreachable!("is_ended() should not be called for this kind of statement"), */
        }
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        match &mut self.kind {
            StatementKind::Extends(statement) => statement.add_item(item),
            StatementKind::Block(statement) => statement.add_item(item),
            StatementKind::If(statement) => statement.add_item(item),
            StatementKind::For(statement) => statement.add_item(item),
            _ => unreachable!("add_item() should not be called for this kind of statement"),
        }
    }

    pub fn get_active_variables(&self) -> HashSet<&'a str> {
        match &self.kind {
            StatementKind::For(statement) => statement.get_active_variables(),
            StatementKind::If(statement) => statement.get_active_variables(),
            _ => HashSet::new(),
        }
    }

    pub fn is_extending(&self) -> bool {
        matches!(&self.kind, StatementKind::Extends(_))
    }
}

impl<'a> From<Statement<'a>> for Item<'a> {
    fn from(statement: Statement<'a>) -> Self {
        Item::Statement(statement)
    }
}

impl ToTokens for Statement<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match &self.kind {
            StatementKind::Extends(extends) => quote! { #extends },

            StatementKind::Block(block) => quote! { #block },
            StatementKind::Parent => {
                let span = self.source.span();
                quote_spanned! {span=> compile_error!("Unexpected 'parent' statement"); }
            }
            StatementKind::EndBlock => {
                let span = self.source.span();
                quote_spanned! {span=> compile_error!("Unexpected 'endblock' statement"); }
            }

            StatementKind::If(statement) => quote! { #statement },
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

            StatementKind::For(statement) => quote! { #statement },
            StatementKind::EndFor => {
                let span = self.source.span();
                quote_spanned! {span=> compile_error!("Unexpected 'endfor' statement"); }
            }
        });
    }
}

pub(super) fn statement<'a>(
    state: &'a State,
    is_extending: &'a bool,
) -> impl Fn(Source) -> Res<Source, (Item, Option<Item>)> + 'a {
    move |input| {
        // Ignore any leading inner whitespace
        let (input, _) = take_while(is_whitespace).parse(input)?;

        // Parse statements
        let (input, mut statement) = context(
            "Expected one of: block, endblock, if, elseif, else, endif, for, endfor",
            cut(alt((
                extends::parse_extends,
                block::parse_block(is_extending),
                block::parse_parent,
                block::parse_endblock,
                r#if::parse_if(state),
                r#if::parse_elseif(state),
                r#if::parse_else,
                r#if::parse_endif,
                r#for::parse_for(state),
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

            // Merge new variables from this statement into the existing local variables
            let is_extending = statement.is_extending();

            loop {
                let mut local_variables = statement.get_active_variables();
                for value in state.local_variables {
                    local_variables.insert(value);
                }

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

                let state = State {
                    local_variables: &local_variables,
                    config: state.config,
                    inferred_escaper_group: state.inferred_escaper_group,
                };

                let (new_input, items) = context(
                    "Failed to parse contents of statement",
                    cut(parse_item(&state, &is_extending)),
                )
                .parse(input)?;
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
}

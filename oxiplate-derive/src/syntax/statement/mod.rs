use std::collections::HashSet;

mod extends;
use extends::Extends;
mod block;
use block::Block;
mod r#for;
use r#for::For;
mod r#if;
use r#if::{ElseIf, If};

use super::{Item, Res, Static};
use crate::syntax::item::tag_end;
use crate::syntax::template::{is_whitespace, parse_item};
use crate::{Source, State};
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::combinator::{cut, fail};
use nom::error::context;
use nom::sequence::preceded;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

#[derive(Debug)]
pub(crate) struct Statement<'a> {
    source: Source<'a>,
    pub(crate) kind: StatementKind<'a>,
}

#[derive(Debug)]
pub(crate) enum StatementKind<'a> {
    Extends(Extends<'a>),
    Block(Block<'a>),
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

    pub fn should_output_blocks(&self) -> bool {
        !matches!(&self.kind, StatementKind::Extends(_))
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
    should_output_blocks: &'a bool,
) -> impl Fn(Source) -> Res<Source, (Item, Option<Static>)> + 'a {
    move |input| {
        // Ignore any leading inner whitespace
        let (input, _) = take_while(is_whitespace)(input)?;

        // Parse statements
        let (input, mut statement) = context(
            "Expected one of: block, endblock, if, elseif, else, endif, for, endfor",
            cut(alt((
                extends::parse_extends,
                block::parse_block(should_output_blocks),
                block::parse_endblock,
                r#if::parse_if(state),
                r#if::parse_elseif(state),
                r#if::parse_else,
                r#if::parse_endif,
                r#for::parse_for(state),
                r#for::parse_endfor,
            ))),
        )(input)?;

        // Parse the closing tag and any trailing whitespace
        let (mut input, mut trailing_whitespace) =
            preceded(take_while(is_whitespace), cut(tag_end("%}")))(input)?;

        if !statement.is_ended(input.as_str().is_empty()) {
            // Append trailing whitespace
            if let Some(trailing_whitespace) = trailing_whitespace {
                statement.add_item(trailing_whitespace.into());
            }
            trailing_whitespace = None;

            // Merge new variables from this statement into the existing local variables
            let should_output_blocks = statement.should_output_blocks();

            loop {
                let mut local_variables = statement.get_active_variables();
                for value in state.local_variables {
                    local_variables.insert(value);
                }
                let state = State {
                    local_variables: &local_variables,
                    config: state.config,
                };

                let (new_input, items) = parse_item(&state, &should_output_blocks)(input)?;
                input = new_input;
                for item in items {
                    if statement.is_ended(false) {
                        if let Item::Whitespace(whitespace) = item {
                            trailing_whitespace = Some(whitespace);
                            continue;
                        }
                    }

                    statement.add_item(item);
                }

                let is_eof = input.as_str().is_empty();
                if statement.is_ended(is_eof) {
                    break;
                } else if is_eof {
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
                        | StatementKind::EndBlock
                        | StatementKind::ElseIf(_)
                        | StatementKind::Else
                        | StatementKind::EndIf
                        | StatementKind::EndFor => unreachable!(
                            "These blocks should never fail to be closed because of EOF"
                        ),
                    };
                    return context(context_message, fail)(input);
                }
            }
        }

        // Return the statement and trailing whitespace
        Ok((input, (statement.into(), trailing_whitespace)))
    }
}

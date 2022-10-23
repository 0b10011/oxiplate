use std::collections::HashSet;

mod r#for;
use r#for::For;
mod r#if;
use r#if::{ElseIf, If};

use super::{Item, Res, Static};
use crate::syntax::item::tag_end;
use crate::syntax::template::{is_whitespace, parse_item};
use crate::Source;
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::combinator::cut;
use nom::combinator::fail;
use nom::error::context;
use nom::sequence::preceded;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

#[derive(Debug)]
pub(crate) struct Statement<'a> {
    source: Source<'a>,
    kind: StatementKind<'a>,
}

#[derive(Debug)]
pub(crate) enum StatementKind<'a> {
    If(If<'a>),
    ElseIf(ElseIf<'a>),
    Else,
    EndIf,
    For(For<'a>),
    EndFor,
}

impl<'a> Statement<'a> {
    pub fn is_ended(&self) -> bool {
        match &self.kind {
            StatementKind::If(statement) => statement.is_ended,
            StatementKind::For(statement) => statement.is_ended,
            _ => true,
        }
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        match &mut self.kind {
            StatementKind::If(statement) => statement.add_item(item),
            StatementKind::For(statement) => statement.add_item(item),
            _ => unreachable!(),
        }
    }

    pub fn get_active_variables(&self) -> HashSet<&'a str> {
        match &self.kind {
            StatementKind::For(statement) => statement.get_active_variables(),
            _ => HashSet::new(),
        }
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
    local_variables: &'a HashSet<&'a str>,
) -> impl Fn(Source) -> Res<Source, (Item, Option<Static>)> + 'a {
    |input| {
        // Ignore any leading inner whitespace
        let (input, _) = take_while(is_whitespace)(input)?;

        // Parse statements
        let (input, mut statement) = context(
            "Expected one of: if, elseif, else, endif, for, endfor",
            cut(alt((
                r#if::parse_if(local_variables),
                r#if::parse_elseif(local_variables),
                r#if::parse_else,
                r#if::parse_endif,
                r#for::parse_for(local_variables),
                r#for::parse_endfor,
            ))),
        )(input)?;

        // Parse the closing tag and any trailing whitespace
        let (mut input, trailing_whitespace) =
            preceded(take_while(is_whitespace), cut(tag_end("%}")))(input)?;

        if !statement.is_ended() {
            // Merge new variables from this statement into the existing local variables
            let mut new_local_variables = statement.get_active_variables();
            for value in local_variables.iter() {
                new_local_variables.insert(value);
            }
            let local_variables = new_local_variables;

            loop {
                let parsed_item = parse_item(&local_variables)(input);
                if parsed_item.is_err() {
                    return context("This statement is never closed.", fail)(statement.source);
                }
                let (new_input, items) =
                    parsed_item.expect("Error possibility should have been checked already");
                input = new_input;
                for item in items {
                    if statement.is_ended() {
                        todo!("This can happen with tags and trailing whitespace I think");
                    }

                    statement.add_item(item);
                }
                if statement.is_ended() {
                    break;
                }
            }
            drop(local_variables);
        }

        // Return the statement and trailing whitespace
        Ok((input, (statement.into(), trailing_whitespace)))
    }
}

use super::super::{expression::expression, Item, Res};
use super::{Statement, StatementKind};
use crate::syntax::template::is_whitespace;
use crate::syntax::Expression;
use crate::Source;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::combinator::cut;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use std::collections::HashSet;

#[derive(Debug)]
pub(crate) struct If<'a> {
    pub ifs: Vec<(Expression<'a>, Vec<Item<'a>>)>,
    pub otherwise: Option<Vec<Item<'a>>>,
    pub is_ended: bool,
}

impl<'a> If<'a> {
    pub fn add_item(&mut self, item: Item<'a>) {
        match item {
            Item::Statement(Statement {
                kind: StatementKind::ElseIf(ElseIf(expression)),
                source: _,
            }) => {
                if self.is_ended {
                    todo!();
                }
                if self.otherwise.is_some() {
                    todo!();
                }

                self.ifs.push((expression, vec![]));
            }
            Item::Statement(Statement {
                kind: StatementKind::Else,
                source: _,
            }) => {
                if self.is_ended {
                    todo!();
                }
                if self.otherwise.is_some() {
                    todo!();
                }

                self.otherwise = Some(vec![]);
            }
            Item::Statement(Statement {
                kind: StatementKind::EndIf,
                source: _,
            }) => {
                self.is_ended = true;
            }
            _ => {
                if self.is_ended {
                    todo!();
                }
                if let Some(items) = &mut self.otherwise {
                    items.push(item);
                } else {
                    self.ifs.last_mut().unwrap().1.push(item);
                }
            }
        }
    }
}

impl<'a> From<If<'a>> for StatementKind<'a> {
    fn from(statement: If<'a>) -> Self {
        StatementKind::If(statement)
    }
}

impl ToTokens for If<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut is_elseif = false;
        for (expression, items) in &self.ifs {
            let if_or_elseif = if !is_elseif {
                quote! { if }
            } else {
                quote! { else if }
            };
            is_elseif = true;
            tokens.append_all(quote! { #if_or_elseif #expression { #(#items);* } });
        }
        if let Some(items) = &self.otherwise {
            tokens.append_all(quote! { else { #(#items)* } });
        }
    }
}

pub(super) fn parse_if<'a>(
    is_extending: &'a bool,
    local_variables: &'a HashSet<&'a str>,
) -> impl FnMut(Source) -> Res<Source, Statement> + 'a {
    |input| {
        let (input, statement_source) = tag("if")(input)?;

        // Consume at least one whitespace.
        let (input, _) = cut(take_while1(is_whitespace))(input)?;

        let (input, output) = cut(expression(is_extending, local_variables))(input)?;

        Ok((
            input,
            Statement {
                kind: If {
                    ifs: vec![(output, vec![])],
                    otherwise: None,
                    is_ended: false,
                }
                .into(),
                source: statement_source,
            },
        ))
    }
}

pub(super) fn parse_elseif<'a>(
    is_extending: &'a bool,
    local_variables: &'a HashSet<&'a str>,
) -> impl Fn(Source) -> Res<Source, Statement> + 'a {
    |input| {
        let (input, statement_source) = tag("elseif")(input)?;

        // Consume at least one whitespace.
        let (input, _) = cut(take_while1(is_whitespace))(input)?;

        let (input, output) = cut(expression(is_extending, local_variables))(input)?;

        Ok((
            input,
            Statement {
                kind: ElseIf(output).into(),
                source: statement_source,
            },
        ))
    }
}

pub(super) fn parse_else(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("else")(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::Else,
            source: output,
        },
    ))
}

pub(super) fn parse_endif(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("endif")(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::EndIf,
            source: output,
        },
    ))
}

#[derive(Debug, PartialEq, Eq)]
pub struct ElseIf<'a>(Expression<'a>);

impl<'a> From<ElseIf<'a>> for StatementKind<'a> {
    fn from(statement: ElseIf<'a>) -> Self {
        StatementKind::ElseIf(statement)
    }
}

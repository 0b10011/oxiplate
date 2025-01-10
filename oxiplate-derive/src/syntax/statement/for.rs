use std::collections::HashSet;

use nom::bytes::complete::{tag, take_while1};
use nom::combinator::cut;
use nom::error::context;
use nom::sequence::tuple;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

use super::super::expression::{expression, ident, keyword, Identifier, Keyword};
use super::super::{Item, Res};
use super::{State, Statement, StatementKind};
use crate::syntax::expression::ExpressionAccess;
use crate::syntax::template::{is_whitespace, Template};
use crate::Source;

#[derive(Debug)]
pub struct For<'a> {
    #[allow(clippy::struct_field_names)]
    for_keyword: Keyword<'a>,
    ident: Identifier<'a>,
    in_keyword: Keyword<'a>,
    expression: ExpressionAccess<'a>,
    template: Template<'a>,
    otherwise: Option<Template<'a>>,
    pub(super) is_ended: bool,
}

impl<'a> For<'a> {
    pub(crate) fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            todo!();
        }

        match item {
            Item::Statement(Statement {
                kind: StatementKind::Else,
                ..
            }) => {
                if self.is_ended {
                    todo!();
                }
                if self.otherwise.is_some() {
                    todo!();
                }

                self.otherwise = Some(Template(vec![]));
            }
            Item::Statement(Statement {
                kind: StatementKind::EndFor,
                ..
            }) => {
                if self.is_ended {
                    todo!();
                }

                self.is_ended = true;
            }
            _ => {
                if let Some(otherwise) = &mut self.otherwise {
                    otherwise.0.push(item);
                } else {
                    self.template.0.push(item);
                }
            }
        }
    }

    pub(crate) fn get_active_variables(&self) -> HashSet<&'a str> {
        HashSet::from([self.ident.ident])
    }
}

impl<'a> From<For<'a>> for StatementKind<'a> {
    fn from(statement: For<'a>) -> Self {
        StatementKind::For(statement)
    }
}

impl ToTokens for For<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let For {
            for_keyword,
            ident,
            in_keyword,
            expression,
            template,
            otherwise,
            is_ended: _,
        } = self;

        if otherwise.is_none() {
            tokens.append_all(quote! { #for_keyword #ident #in_keyword #expression { #template } });
        } else {
            tokens.append_all(quote! {
                {
                    let mut loop_ran = false;
                    #for_keyword #ident #in_keyword #expression {
                        loop_ran = true;
                        #template
                    }
                    if !loop_ran {
                        #otherwise
                    }
                }
            });
        }
    }
}

pub(super) fn parse_for<'a>(state: &'a State) -> impl Fn(Source) -> Res<Source, Statement> + 'a {
    |input| {
        let (input, for_keyword) = keyword("for")(input)?;

        let (input, (_, ident, _, in_keyword, _, expression)) = cut(tuple((
            context("Expected space after 'for'", take_while1(is_whitespace)),
            context("Expected an identifier", ident),
            context(
                "Expected space after identifier",
                take_while1(is_whitespace),
            ),
            context("Expected 'in'", keyword("in")),
            context("Expected space after 'in'", take_while1(is_whitespace)),
            context(
                "Expected an expression that is iterable",
                expression(state, true, true),
            ),
        )))(input)?;

        let source = for_keyword.0.clone();

        Ok((
            input,
            Statement {
                kind: For {
                    for_keyword,
                    ident,
                    in_keyword,
                    expression,
                    template: Template(vec![]),
                    otherwise: None,
                    is_ended: false,
                }
                .into(),
                source,
            },
        ))
    }
}

pub(super) fn parse_endfor(input: Source) -> Res<Source, Statement> {
    let (input, output) = tag("endfor")(input)?;

    Ok((
        input,
        Statement {
            kind: StatementKind::EndFor,
            source: output,
        },
    ))
}

use super::super::expression::{ident, keyword, Identifier, Keyword};
use super::super::{expression::expression, Item, Res};
use super::{State, Statement, StatementKind};
use crate::syntax::expression::ExpressionAccess;
use crate::syntax::template::{is_whitespace, Template};
use crate::Source;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::combinator::cut;
use nom::error::context;
use nom::sequence::tuple;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use std::collections::HashSet;

#[derive(Debug)]
pub struct For<'a> {
    #[allow(clippy::struct_field_names)]
    for_keyword: Keyword<'a>,
    ident: Identifier<'a>,
    in_keyword: Keyword<'a>,
    expression: ExpressionAccess<'a>,
    template: Template<'a>,
    pub(super) is_ended: bool,
}

impl<'a> For<'a> {
    pub(crate) fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            todo!();
        }

        match item {
            Item::Statement(Statement {
                kind: StatementKind::EndFor,
                ..
            }) => {
                self.is_ended = true;
            }
            _ => {
                self.template.0.push(item);
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
            ..
        } = self;
        tokens.append_all(quote! { #for_keyword #ident #in_keyword #expression { #template } });
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
                expression(state, true),
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

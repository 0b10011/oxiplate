use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::{cut, opt};
use nom::error::context;
use proc_macro::{Diagnostic, Level};
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote};

use super::super::Item;
use super::{Statement, StatementKind};
use crate::syntax::Res;
use crate::syntax::expression::{ExpressionAccess, expression};
use crate::syntax::statement::r#if::{Type, parse_type};
use crate::syntax::template::{Template, whitespace};
use crate::{Source, State};

#[derive(Debug)]
pub(crate) struct Match<'a> {
    expression: ExpressionAccess<'a>,

    /// Errors for attempting to add items before the first case.
    errors: Template<'a>,
    cases: Vec<Case<'a>>,
    is_ended: bool,
}

impl<'a> Match<'a> {
    pub fn is_ended(&self) -> bool {
        self.is_ended
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        if self.is_ended {
            Diagnostic::spanned(
                item.source().span().unwrap(),
                Level::Error,
                "Internal Oxiplate error: Attempted to add item to ended `match` statement.",
            )
            .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Attempted+to+add+item+to+ended+match+statement")
            .help("Include template that caused the issue and the associated note.")
            .emit();
            unreachable!("Internal Oxiplate error. See previous error for more information.");
        }

        match item {
            Item::Statement(Statement {
                kind: StatementKind::Case(case),
                source: _,
            }) => {
                self.cases.push(case);
            }
            Item::Statement(Statement {
                kind: StatementKind::EndMatch,
                source: _,
            }) => {
                self.is_ended = true;
            }
            Item::Comment(_) | Item::Whitespace(_) => {
                if let Some(case) = self.cases.last_mut() {
                    case.add_item(item);
                } else {
                    // Discard comments and whitespace outside of a case
                }
            }
            Item::CompileError { .. } => {
                self.errors.0.push(item);
            }
            _ => {
                if let Some(case) = self.cases.last_mut() {
                    case.add_item(item);
                } else {
                    let error_source = item.source().clone();
                    let consumed_source = item.source().clone();
                    self.errors.0.push(Item::CompileError {
                        message: format!("Expected `case` or `endmatch` statement {item:?}"),
                        error_source,
                        consumed_source,
                    });
                }
            }
        }
    }

    pub(crate) fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let mut tokens = TokenStream::new();
        let mut estimated_length = usize::MAX;

        let mut cases = TokenStream::new();
        for case in &self.cases {
            let mut local_variables = case.ty.get_variables();
            for value in state.local_variables {
                local_variables.insert(value);
            }
            let branch_state = &State {
                local_variables: &local_variables,
                ..*state
            };

            let (case, case_length) = case.to_tokens(branch_state);
            estimated_length = estimated_length.min(case_length);
            cases.append_all(case);
        }

        let (expression, _expression_length) = self.expression.to_tokens(state);
        let (errors, _errors_length) = self.errors.to_tokens(state);

        tokens.append_all(quote! { #errors match #expression { #cases } });

        (tokens, estimated_length)
    }

    pub fn parse(input: Source) -> Res<Source, Statement> {
        let (input, (statement, leading_whitespace, expression)) = (
            tag("match"),
            opt(whitespace),
            context(
                r#"Expected an expression after "match""#,
                cut(expression(true, true)),
            ),
        )
            .parse(input)?;

        let source = statement
            .merge_some(
                leading_whitespace.as_ref(),
                "Whitespace expected after `match`",
            )
            .merge(&expression.source(), "Expression expected after whitespace");

        Ok((
            input,
            Statement {
                kind: StatementKind::Match(Match {
                    expression,
                    errors: Template(vec![]),
                    cases: vec![],
                    is_ended: false,
                }),
                source,
            },
        ))
    }

    pub(super) fn parse_end(input: Source) -> Res<Source, Statement> {
        let (input, output) = tag("endmatch").parse(input)?;

        Ok((
            input,
            Statement {
                kind: StatementKind::EndMatch,
                source: output,
            },
        ))
    }
}

#[derive(Debug)]
pub(crate) struct Case<'a> {
    ty: Type<'a>,
    template: Template<'a>,
}

impl<'a> Case<'a> {
    pub fn parse(input: Source) -> Res<Source, Statement> {
        let (input, (statement, leading_whitespace, ty)) = (
            tag("case"),
            opt(whitespace),
            context("Expected a type after `case`", cut(parse_type)),
        )
            .parse(input)?;

        let source = statement
            .merge_some(
                leading_whitespace.as_ref(),
                "Whitespace expected after `match`",
            )
            .merge(ty.source(), "Type expected after whitespace");

        Ok((
            input,
            Statement {
                kind: StatementKind::Case(Case {
                    ty,
                    template: Template(vec![]),
                }),
                source,
            },
        ))
    }

    pub fn to_tokens(&self, state: &State) -> (TokenStream, usize) {
        let ty = self.ty.to_tokens(state);
        let (template, estimated_length) = self.template.to_tokens(state);
        let tokens = quote! { #ty => { #template } };

        (tokens, estimated_length)
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        self.template.0.push(item);
    }
}

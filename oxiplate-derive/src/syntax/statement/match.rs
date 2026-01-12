use std::collections::HashSet;

use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::{cut, opt};
use nom::error::context;
use nom::multi::many0;
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote, quote_spanned};

use super::super::Item;
use super::{Statement, StatementKind};
use crate::syntax::Res;
use crate::syntax::expression::{ExpressionAccess, expression};
use crate::syntax::statement::helpers::pattern::Pattern;
use crate::syntax::template::{Template, whitespace};
use crate::{Source, State, Tokens, internal_error};

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
            internal_error!(
                item.source().span().unwrap(),
                "Attempted to add item to ended `match` statement",
            );
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
                        message: "Expected `case` or `endmatch` statement".to_string(),
                        error_source,
                        consumed_source,
                    });
                }
            }
        }
    }

    pub(crate) fn to_tokens(&self, state: &mut State) -> Tokens {
        let mut tokens = TokenStream::new();
        let mut estimated_length = usize::MAX;

        let mut cases = TokenStream::new();
        for case in &self.cases {
            let (case, case_length) = case.to_tokens(state);
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
    first_pattern: Pattern<'a>,
    additional_patterns: Vec<(Source<'a>, Pattern<'a>)>,
    guard: Option<Guard<'a>>,
    template: Template<'a>,
}

impl<'a> Case<'a> {
    pub fn parse(input: Source) -> Res<Source, Statement> {
        let (
            input,
            (
                statement,
                leading_whitespace,
                (first_pattern, additional_patterns_and_whitespace, guard),
            ),
        ) = (
            tag("case"),
            opt(whitespace),
            context(
                "Expected a match pattern after `case`",
                cut((
                    Pattern::parse,
                    many0((opt(whitespace), tag("|"), opt(whitespace), Pattern::parse)),
                    opt((whitespace, Guard::parse)),
                )),
            ),
        )
            .parse(input)?;

        let mut source = statement
            .merge_some(
                leading_whitespace.as_ref(),
                "Whitespace expected after `match`",
            )
            .merge(first_pattern.source(), "Type expected after whitespace");

        let mut additional_patterns = Vec::with_capacity(additional_patterns_and_whitespace.len());
        for (leading_whitespace, operator, middle_whitespace, pattern) in
            additional_patterns_and_whitespace
        {
            source = source
                .merge_some(
                    leading_whitespace.as_ref(),
                    "Whitespace expected after pattern",
                )
                .merge(&operator, "`|` expected after whitespace")
                .merge_some(middle_whitespace.as_ref(), "Whitespace expected after `|`")
                .merge(pattern.source(), "Pattern expected after whitespace");

            additional_patterns.push((operator, pattern));
        }

        let guard = if let Some((leading_whitespace, guard)) = guard {
            source = source
                .merge(&leading_whitespace, "Whitespace expected after patterns")
                .merge(guard.source(), "Guard expected after whitespace");
            Some(guard)
        } else {
            None
        };

        Ok((
            input,
            Statement {
                kind: StatementKind::Case(Case {
                    first_pattern,
                    additional_patterns,
                    guard,
                    template: Template(vec![]),
                }),
                source,
            },
        ))
    }

    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        let mut vars: HashSet<&'a str> = self.first_pattern.get_variables();

        for (_operator, pattern) in &self.additional_patterns {
            vars.extend(pattern.get_variables());
        }

        vars
    }

    pub fn to_tokens<'b: 'a>(&self, state: &mut State<'b>) -> Tokens {
        let mut tokens = self.first_pattern.to_tokens(state);

        state.local_variables.push_stack();

        for (operator, pattern) in &self.additional_patterns {
            let span = operator.span();
            let pattern = pattern.to_tokens(state);
            tokens.append_all(quote_spanned! {span=> | #pattern });
        }

        state.local_variables.add(
            self.get_variables()
                .iter()
                .map(ToString::to_string)
                .collect(),
        );

        if let Some(guard) = &self.guard {
            tokens.append_all(guard.to_tokens(state));
        }

        let (template, estimated_length) = self.template.to_tokens(state);
        tokens.append_all(quote! { => { #template } });

        state.local_variables.pop_stack();

        (tokens, estimated_length)
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        self.template.0.push(item);
    }
}

#[derive(Debug)]
/// See: <https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#adding-conditionals-with-match-guards>
struct Guard<'a> {
    if_tag: Source<'a>,
    expression: ExpressionAccess<'a>,
    source: Source<'a>,
}

impl<'a> Guard<'a> {
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, (if_tag, middle_whitespace, expression)) = (
            tag("if"),
            whitespace,
            context(
                "Expected expression after `if`",
                cut(expression(true, true)),
            ),
        )
            .parse(input)?;

        let source = if_tag
            .clone()
            .merge(&middle_whitespace, "Whitespace expected after `if`")
            .merge(&expression.source(), "Expression expected after whitespace");

        Ok((
            input,
            Self {
                if_tag,
                expression,
                source,
            },
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let if_span = self.if_tag.span();
        let (expression, _estimated_length) = self.expression.to_tokens(state);

        quote_spanned! {if_span=> if #expression }
    }
}

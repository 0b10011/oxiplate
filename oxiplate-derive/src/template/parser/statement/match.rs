use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, TokenStreamExt};

use super::super::Item;
use super::{Statement, StatementKind};
use crate::parser::{cut, many0, opt, take, Parser as _};
use crate::template::parser::expression::{expression, ExpressionAccess, KeywordParser};
use crate::template::parser::r#static::StaticType;
use crate::template::parser::statement::helpers::pattern::Pattern;
use crate::template::parser::template::Template;
use crate::template::parser::Res;
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{internal_error, BuiltTokens, Source, State};

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
                item.source().span_token().unwrap(),
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
            Item::Comment(_) | Item::Whitespace(_) | Item::Static(_, StaticType::Whitespace) => {
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

    pub(crate) fn to_tokens(&self, state: &mut State) -> BuiltTokens {
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

    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Statement<'a>> {
        let (tokens, (statement, expression)) = (
            KeywordParser::new("match"),
            cut(
                r#"Expected an expression after "match""#,
                expression(true, true),
            ),
        )
            .parse(tokens)?;

        let source = statement
            .source()
            .clone()
            .merge(&expression.source(), "Expression expected after `match`");

        Ok((
            tokens,
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

    pub(super) fn parse_end(tokens: TokenSlice<'a>) -> Res<'a, Statement<'a>> {
        let (tokens, output) = KeywordParser::new("endmatch").parse(tokens)?;

        Ok((
            tokens,
            Statement {
                kind: StatementKind::EndMatch,
                source: output.source().clone(),
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
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Statement<'a>> {
        let (tokens, (statement, (first_pattern, additional_patterns_and_separators, guard))) = (
            KeywordParser::new("case"),
            cut(
                "Expected a match pattern after `case`",
                (
                    Pattern::parse,
                    many0((take(TokenKind::VerticalBar), Pattern::parse)),
                    opt(Guard::parse),
                ),
            ),
        )
            .parse(tokens)?;

        let mut source = statement
            .source()
            .clone()
            .merge(first_pattern.source(), "Type expected after whitespace");

        let mut additional_patterns = Vec::with_capacity(additional_patterns_and_separators.len());
        for (operator, pattern) in additional_patterns_and_separators {
            source = source
                .merge(operator.source(), "`|` expected after previous pattern")
                .merge(pattern.source(), "Pattern expected after `|`");

            additional_patterns.push((operator.source().clone(), pattern));
        }

        let guard = if let Some(guard) = guard {
            source = source.merge(guard.source(), "Guard expected after whitespace");
            Some(guard)
        } else {
            None
        };

        Ok((
            tokens,
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

    pub fn to_tokens<'b: 'a>(&self, state: &mut State<'b>) -> BuiltTokens {
        let mut tokens = self.first_pattern.to_tokens(state);

        state.local_variables.push_stack();

        for (operator, pattern) in &self.additional_patterns {
            let span = operator.span_token();
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
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (if_tag, expression)) = (
            KeywordParser::new("if"),
            cut("Expected expression after `if`", expression(true, true)),
        )
            .parse(tokens)?;

        let source = if_tag
            .source()
            .clone()
            .merge(&expression.source(), "Expression expected after whitespace");

        Ok((
            tokens,
            Self {
                if_tag: if_tag.source().clone(),
                expression,
                source,
            },
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let if_span = self.if_tag.span_token();
        let (expression, _estimated_length) = self.expression.to_tokens(state);

        quote_spanned! {if_span=> if #expression }
    }
}

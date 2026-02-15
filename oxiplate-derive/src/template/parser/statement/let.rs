use std::collections::HashSet;

use quote::quote_spanned;

use super::super::expression::{Keyword, expression};
use super::{State, Statement, StatementKind};
use crate::parser::{Parser as _, cut, take};
use crate::template::parser::Res;
use crate::template::parser::expression::{ExpressionAccess, KeywordParser};
use crate::template::parser::statement::helpers::pattern::Pattern;
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{BuiltTokens, Source};

/// `let` statement for saving values to variables.
#[derive(Debug)]
pub(crate) struct Let<'a> {
    /// `let` keyword
    keyword: Keyword<'a>,

    /// Pattern for destructuring
    pattern: Pattern<'a>,

    /// `=` operator
    operator: Source<'a>,

    /// Value to save to the variable
    expr: ExpressionAccess<'a>,

    /// Source for the entire statement
    source: Source<'a>,
}

impl<'a> Let<'a> {
    /// Attempt to parse a `let` statement from the current input.
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (keyword, pattern, operator, expr)) = (
            KeywordParser::new("let"),
            cut("Expected a pattern", Pattern::parse),
            cut("Expected `=`", take(TokenKind::Equal)),
            cut("Expected an expression", expression(true, true)),
        )
            .parse(tokens)?;

        let source = keyword
            .source()
            .clone()
            .merge(pattern.source(), "Variable name expected after `let`")
            .merge(operator.source(), "`=` expected after variable name")
            .merge(&expr.source(), "Expression expected after expression");

        Ok((
            tokens,
            Self {
                keyword,
                pattern,
                operator: operator.source().clone(),
                expr,
                source,
            },
        ))
    }

    /// Get the `Source` for the statement.
    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    /// Get variable names.
    pub fn variables(&'a self) -> HashSet<&'a str> {
        self.pattern.get_variables()
    }

    /// Build token stream for the statement.
    pub fn to_tokens(&self, state: &State) -> BuiltTokens {
        let span = self.source.span_token();
        let keyword = &self.keyword;
        let pattern = self.pattern.to_tokens(state);
        let operator_span = self.operator.span_token();
        let operator = quote_spanned! {operator_span=> = };
        let (expr, _estimated_length) = self.expr.to_tokens(state);

        (
            quote_spanned! {span=> #keyword #pattern #operator #expr; },
            0,
        )
    }
}

impl<'a> From<Let<'a>> for Statement<'a> {
    fn from(value: Let<'a>) -> Self {
        Statement {
            source: value.source().clone(),
            kind: StatementKind::Let(value),
        }
    }
}

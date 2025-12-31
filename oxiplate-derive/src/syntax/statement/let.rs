use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use quote::quote_spanned;

use super::super::Res;
use super::super::expression::{Identifier, Keyword, expression, keyword};
use super::{State, Statement, StatementKind};
use crate::syntax::expression::ExpressionAccess;
use crate::syntax::template::whitespace;
use crate::{Source, Tokens};

/// `let` statement for saving values to variables.
#[derive(Debug)]
pub(crate) struct Let<'a> {
    /// `let` keyword
    keyword: Keyword<'a>,

    /// Variable name
    ident: Identifier<'a>,

    /// `=` operator
    operator: Source<'a>,

    /// Value to save to the variable
    expr: ExpressionAccess<'a>,

    /// Source for the entire statement
    source: Source<'a>,
}

impl<'a> Let<'a> {
    /// Attempt to parse a `let` statement from the current input.
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (
            input,
            (
                keyword,
                leading_whitespace,
                ident,
                middle_whitespace,
                operator,
                trailing_whitespace,
                expr,
            ),
        ) = (
            keyword("let"),
            opt(whitespace),
            Identifier::parse,
            opt(whitespace),
            tag("="),
            opt(whitespace),
            expression(true, true),
        )
            .parse(input)?;

        let source = keyword
            .0
            .clone()
            .merge_some(
                leading_whitespace.as_ref(),
                "Whitespace expected after `let`",
            )
            .merge(ident.source(), "Variable name expected after whitespace")
            .merge_some(
                middle_whitespace.as_ref(),
                "Whitespace expected after variable name",
            )
            .merge(&operator, "`=` expected after whitespace")
            .merge_some(
                trailing_whitespace.as_ref(),
                "Whitespace expected after `=`",
            )
            .merge(&expr.source(), "Expression expected after whitespace");

        Ok((
            input,
            Self {
                keyword,
                ident,
                operator,
                expr,
                source,
            },
        ))
    }

    /// Get the `Source` for the statement.
    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    /// Get the variable name.
    pub fn variable(&self) -> &'a str {
        self.ident.as_str()
    }

    /// Build token stream for the statement.
    pub fn to_tokens(&self, state: &State) -> Tokens {
        let span = self.source.span();
        let keyword = &self.keyword;
        let ident = &self.ident;
        let operator_span = self.operator.span();
        let operator = quote_spanned! {operator_span=> = };
        let (expr, _estimated_length) = self.expr.to_tokens(state);

        (quote_spanned! {span=> #keyword #ident #operator #expr; }, 0)
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

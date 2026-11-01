use std::collections::HashSet;

use super::helpers::pattern::Pattern;
use super::{State, Statement, StatementKind};
use crate::template::parser::expression::{Expression, Keyword, KeywordParser, expression};
use crate::template::parser::prelude::*;

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
    expr: Expression<'a>,

    /// Source for the entire statement
    source: Source<'a>,
}

impl<'a> Let<'a> {
    /// Attempt to parse a `let` statement from the current input.
	pub(in crate::template::parser) fn parser() -> LetParser<'a> {
	    LetParser {
	        phantom_data: PhantomData,
	    }
	}

    /// Get the `Source` for the statement.
    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    /// Get variable names.
    pub fn variables(&'a self) -> HashSet<&'a str> {
        self.pattern.get_variables()
    }
}

impl<'a> ToTokensWithMutState<'a> for Let<'a> {
    fn to_tokens_with_mut_state<'b: 'a>(&'a self, state: &mut State<'b>) -> BuiltTokens {
        let span = self.source.span_token();
        let keyword = &self.keyword;
        let (pattern, _estimated_length) = self.pattern.to_tokens_with_state(state);
        let operator_span = self.operator.span_token();
        let operator = quote_spanned! {operator_span=> = };
        let (expr, _estimated_length) = self.expr.to_tokens_with_state(state);

        (
            quote_spanned! {span=> #keyword #pattern #operator #expr; },
            EstimatedLength::new(0),
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

pub(in crate::template::parser) struct LetParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for LetParser<'a> {
    type TokenKind = TokenKind;
    type Output = Let<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (keyword, pattern, operator, expr)) = (
            KeywordParser::new("let"),
            cut("Expected a pattern", Pattern::parser()),
            cut("Expected `=`", take(TokenKind::Equal)),
            cut("Expected an expression after `=`", expression(true)),
        )
            .parse(tokens)?;

        let source = keyword
            .source()
            .clone()
            .merge(&pattern.source(), "Variable name expected after `let`")
            .merge(operator.source(), "`=` expected after variable name")
            .merge(&expr.source(), "Expression expected after expression");

        Ok((
            tokens,
            Let {
                keyword,
                pattern,
                operator: operator.source().clone(),
                expr,
                source,
            },
        ))
    }
}

use super::operator::Operator;
use super::{Expression, NestedExpression, expression, parse_operator};
use crate::template::parser::prelude::*;

/// A calculation (`a + b`).
#[derive(Debug)]
pub(crate) struct Calc<'a> {
    pub(super) left: Box<Expression<'a>>,
    operator: Operator<'a>,
    pub(super) right: Option<Box<Expression<'a>>>,
}

impl<'a> Calc<'a> {
    /// Parses a calculation (`a + b`).
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Box<NestedExpression<'a>>> {
        let (tokens, operator) = parse_operator.parse(tokens)?;

        let (tokens, right) = if operator.requires_expression_after() {
            let (tokens, expression) =
                cut("Expected an expression", expression(false)).parse(tokens)?;
            (tokens, Some(expression))
        } else {
            ignore_recoverable_errors(expression(false)).parse(tokens)?
        };

        let callback = Box::new(|left: Expression<'a>| -> Expression<'a> {
            Expression::Calc(Calc {
                left: Box::new(left),
                operator,
                right: right.map(Box::new),
            })
        });

        Ok((tokens, callback))
    }

    /// Builds source for entire expression.
    pub fn source(&self) -> Source<'a> {
        self.left
            .source()
            .merge(self.operator.source(), "Operator should follow whitespace")
            .merge_some(
                self.right.as_deref().map(Expression::source).as_ref(),
                "Right expression should follow whitespace",
            )
    }
}

impl<'a> ToTokensWithState<'a> for Calc<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let (left, left_length) = self.left.to_tokens_with_state(state);
        let operator = &self.operator;
        let (right, right_length) = if let Some(right) = self.right.as_ref() {
            right.to_tokens_with_state(state)
        } else {
            (TokenStream::new(), left_length)
        };

        (
            quote! { #left #operator #right },
            left_length.min(right_length),
        )
    }
}

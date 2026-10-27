use super::arguments::{ArgumentsGroup, arguments};
use super::path::PathExprSegment;
use super::{Expression, NestedExpression};
use crate::template::parser::prelude::*;

/// A method call expression (`expr.method(args)`).
/// See: <https://doc.rust-lang.org/reference/expressions/method-call-expr.html#railroad-MethodCallExpression>
#[derive(Debug)]
pub(in crate::template::parser) struct MethodCall<'a> {
    pub(super) expression: Box<Expression<'a>>,
    period: Source<'a>,
    segment: PathExprSegment<'a>,
    arguments: ArgumentsGroup<'a>,
}

impl<'a> MethodCall<'a> {
    /// Parses a method call expression (`expr.method(args)`).
    /// See: <https://doc.rust-lang.org/reference/expressions/method-call-expr.html#railroad-MethodCallExpression>
    pub(in crate::template::parser) fn parse(
        tokens: TokenSlice<'a>,
    ) -> Res<'a, Box<NestedExpression<'a>>> {
        let (tokens, (period, segment, arguments)) = (
            into(take(TokenKind::Period)),
            PathExprSegment::parse,
            arguments,
        )
            .parse(tokens)?;

        Ok((
            tokens,
            Box::new(|expression: Expression<'a>| {
                Expression::MethodCall(Self {
                    expression: Box::new(expression),
                    period,
                    segment,
                    arguments,
                })
            }),
        ))
    }

    /// Builds source for entire call expression.
    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
        self.expression
            .source()
            .merge(&self.period, "`.` expected after expression")
            .merge(
                &self.segment.source(),
                "Path expression segment expected after `.`",
            )
            .merge(
                &self.arguments.source(),
                "Arguments expected after expression",
            )
    }
}

impl<'a> ToTokensWithState<'a> for MethodCall<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let span = self.period.span_token();
        let (expression, _estimated_length) = self.expression.to_tokens_with_state(state);
        let (segment, _estimated_length) = self.segment.to_tokens_with_state(state);
        let (arguments, _estimated_length) = self.arguments.to_tokens_with_state(state);

        (
            quote_spanned! {span=> #expression . #segment #arguments },
            EstimatedLength::new(1),
        )
    }
}

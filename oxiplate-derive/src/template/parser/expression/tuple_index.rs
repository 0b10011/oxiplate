use proc_macro2::Literal;

use super::{Expression, NestedExpression};
use crate::template::parser::prelude::*;

/// Tuple index expression (`expr.tuple_index`).
///
/// See:
/// - <https://doc.rust-lang.org/reference/expressions/tuple-expr.html#railroad-TupleIndexingExpression>
#[derive(Debug)]
pub(crate) struct TupleIndexingExpression<'a> {
    /// Expression being indexed into.
    pub(super) expression: Box<Expression<'a>>,

    period: Source<'a>,
    index: TupleIndex<'a>,
}

impl<'a> TupleIndexingExpression<'a> {
    /// Parses a tuple index expression (`expr.tuple_index`).
    ///
    /// See:
    /// - <https://doc.rust-lang.org/reference/expressions/tuple-expr.html#railroad-TupleIndexingExpression>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Box<NestedExpression<'a>>> {
        let (tokens, (period, index)) =
            (into(take(TokenKind::Period)), TupleIndex::parse).parse(tokens)?;

        Ok((
            tokens,
            Box::new(|expression: Expression<'a>| {
                Expression::TupleIndex(TupleIndexingExpression {
                    expression: Box::new(expression),
                    period,
                    index,
                })
            }),
        ))
    }

    /// Builds source for entire expression.
    pub fn source(&self) -> Source<'a> {
        self.expression
            .source()
            .merge(&self.period, "`.` should follow expression")
            .merge(self.index.source(), "Index should follow `.`")
    }
}

impl<'a> ToTokensWithState<'a> for TupleIndexingExpression<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let span = self.period.span_token();
        let (expression, estimated_length) = self.expression.to_tokens_with_state(state);
        let (index, _index_length) = self.index.to_tokens_with_state(state);
        (
            quote_spanned! {span=> #expression . #index },
            estimated_length,
        )
    }
}

/// Tuple index.
///
/// See:
/// - <https://doc.rust-lang.org/reference/tokens.html#grammar-TUPLE_INDEX>
#[derive(Debug)]
pub(crate) struct TupleIndex<'a> {
    index: Source<'a>,
    literal: Literal,
}

impl<'a> TupleIndex<'a> {
    /// Parses a tuple index.
    ///
    /// See:
    /// - <https://doc.rust-lang.org/reference/tokens.html#grammar-TUPLE_INDEX>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, index): (_, Source) = (alt((
            into(take(TokenKind::DecLiteral)),
            into(take(TokenKind::BinLiteral)),
            into(take(TokenKind::OctLiteral)),
            into(take(TokenKind::HexLiteral)),
        )))
        .parse(tokens)?;

        let mut literal = match index.as_str().parse::<Literal>() {
            Ok(literal) => literal,
            Err(error) => {
                #[cfg(not(feature = "better-internal-errors"))]
                let _ = error;

                internal_error!(
                    index.span_token().unwrap(),
                    "Failed to parse tuple index as a literal",
                    .span_help(error.span().unwrap(), format!("Original error: {error}")
                ));
            }
        };
        literal.set_span(index.span_token());

        Ok((tokens, Self { index, literal }))
    }

    /// Builds source for entire expression.
    pub fn source(&self) -> &Source<'a> {
        &self.index
    }
}

impl<'a> ToTokensWithState<'a> for TupleIndex<'a> {
    fn to_tokens_with_state(&self, _state: &State<'a>) -> BuiltTokens {
        (self.literal.to_token_stream(), EstimatedLength::new(0))
    }
}

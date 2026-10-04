use super::{Expression, NestedExpression, expression};
use crate::template::parser::prelude::*;

/// Index expression (`expr[expr]`).
///
/// See:
/// - <https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions>
/// - <https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices>
#[derive(Debug)]
#[expect(clippy::struct_field_names)]
pub(crate) struct Index<'a> {
    /// Expression being indexed into.
    pub(super) expression: Box<Expression<'a>>,

    /// Includes brackets and index (`[expr]`).
    brackets: Source<'a>,

    index: Box<Expression<'a>>,
}

impl<'a> Index<'a> {
    /// Parses an index expression (`expr[expr]`).
    ///
    /// See:
    /// - <https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions>
    /// - <https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Box<NestedExpression<'a>>> {
        let (tokens, (open_bracket, index, close_bracket)) = (
            take(TokenKind::OpenBracket),
            cut("Expected an expression", expression(true)),
            cut("Expected `]`", take(TokenKind::CloseBracket)),
        )
            .parse(tokens)?;

        let brackets = open_bracket
            .source()
            .clone()
            .merge(&index.source(), "Index expected after `[`")
            .merge(close_bracket.source(), "`]` expected after index");

        Ok((
            tokens,
            Box::new(|expression: Expression<'a>| {
                Expression::Index(Index {
                    expression: Box::new(expression),
                    brackets,
                    index: Box::new(index),
                })
            }),
        ))
    }

    /// Builds source for entire expression.
    pub fn source(&self) -> Source<'a> {
        self.expression
            .source()
            .merge(&self.brackets, "`[index]` should follow expression")
    }
}

impl<'a> ToTokensWithState<'a> for Index<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let span = self.brackets.span_token();
        let (expression, estimated_length) = self.expression.to_tokens_with_state(state);
        let (range, _range_length) = self.index.to_tokens_with_state(state);
        (
            quote_spanned! {span=> #expression [ #range ] },
            estimated_length,
        )
    }
}

impl<'a> From<Index<'a>> for Expression<'a> {
    fn from(value: Index<'a>) -> Self {
        Self::Index(value)
    }
}

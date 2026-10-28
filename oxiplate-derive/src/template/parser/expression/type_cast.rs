use super::types::no_bounds::TypeNoBounds;
use super::{Expression, KeywordParser, NestedExpression};
use crate::template::parser::prelude::*;

/// A type cast expression (`expr as Type`).
/// See: <https://doc.rust-lang.org/reference/expressions/operator-expr.html#railroad-TypeCastExpression>
#[derive(Debug)]
pub(crate) struct TypeCast<'a> {
    pub(super) expression: Box<Expression<'a>>,
    r#as: Source<'a>,
    type_no_bounds: TypeNoBounds<'a>,
}

impl<'a> TypeCast<'a> {
    /// Parse a type cast expression (`expr as Type`).
    /// See: <https://doc.rust-lang.org/reference/expressions/operator-expr.html#railroad-TypeCastExpression>
    pub(in crate::template::parser) fn parse(
        tokens: TokenSlice<'a>,
    ) -> Res<'a, Box<NestedExpression<'a>>> {
        let (tokens, (r#as, type_no_bounds)) = (
            into(KeywordParser::new("as")),
            cut("Expected type to cast expression into", TypeNoBounds::parse),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            Box::new(|expression: Expression<'a>| {
                Self {
                    expression: Box::new(expression),
                    r#as,
                    type_no_bounds,
                }
                .into()
            }),
        ))
    }

    /// Source for the entire type cast expression.
    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
        self.expression
            .source()
            .merge(&self.r#as, "`as` expected after expression")
            .merge(
                &self.type_no_bounds.source(),
                "Type no bounds expected after `as`",
            )
    }
}

impl<'a> ToTokensWithState<'a> for TypeCast<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let (expression, estimated_length) = self.expression.to_tokens_with_state(state);

        let span = self.r#as.span_token();
        let type_no_bounds = self.type_no_bounds.to_tokens_with_state(state).0;
        (
            quote_spanned! {span=> #expression as #type_no_bounds },
            estimated_length,
        )
    }
}

impl<'a> From<TypeCast<'a>> for Expression<'a> {
    fn from(value: TypeCast<'a>) -> Self {
        Expression::TypeCast(value)
    }
}

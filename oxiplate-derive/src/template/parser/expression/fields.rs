use super::{Expression, Identifier, NestedExpression};
use crate::template::parser::prelude::*;

/// A field or method.
/// See: <https://doc.rust-lang.org/reference/expressions/field-expr.html#railroad-FieldExpression>
#[derive(Debug)]
pub(crate) struct Fields<'a> {
    pub(super) expression: Box<Expression<'a>>,
    period: Source<'a>,
    ident: Identifier<'a>,
}

impl<'a> Fields<'a> {
    /// Parse a field or method.
    /// See: <https://doc.rust-lang.org/reference/expressions/field-expr.html#railroad-FieldExpression>
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Box<NestedExpression<'a>>> {
        let (tokens, (period, ident)) =
            (into(take(TokenKind::Period)), Identifier::parse).parse(tokens)?;

        Ok((
            tokens,
            Box::new(|expression: Expression<'a>| {
                Self {
                    expression: Box::new(expression),
                    period,
                    ident,
                }
                .into()
            }),
        ))
    }

    /// Source for the entire group, including the parentheses.
    pub fn source(&self) -> Source<'a> {
        self.expression
            .source()
            .merge(&self.period, "`.` expected after expression")
            .merge(
                self.ident.source(),
                "Field or method name should immediately follow the dot",
            )
    }
}

impl<'a> ToTokensWithState<'a> for Fields<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let (expression, estimated_length) = self.expression.to_tokens_with_state(state);

        let span = self.period.span_token();
        let ident = self.ident.to_token_stream();
        (
            quote_spanned! {span=> #expression . #ident },
            estimated_length,
        )
    }
}

impl<'a> From<Fields<'a>> for Expression<'a> {
    fn from(value: Fields<'a>) -> Self {
        Expression::Fields(value)
    }
}

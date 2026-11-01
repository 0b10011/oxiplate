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
    /// Parse a field (`expr.ident`).
    /// See: <https://doc.rust-lang.org/reference/expressions/field-expr.html#railroad-FieldExpression>
    pub(in crate::template::parser) fn parser() -> FieldsParser<'a> {
        FieldsParser {
            phantom_data: PhantomData,
        }
    }

    /// Source for the entire field expression.
    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
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

/// Parse a field (`expr.ident`).
/// See: <https://doc.rust-lang.org/reference/expressions/field-expr.html#railroad-FieldExpression>
pub(in crate::template::parser) struct FieldsParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for FieldsParser<'a> {
    type TokenKind = TokenKind;
    type Output = Box<NestedExpression<'a>>;

    /// Parse a field (`expr.ident`).
    /// See: <https://doc.rust-lang.org/reference/expressions/field-expr.html#railroad-FieldExpression>
    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (period, ident)) =
            (into(take(TokenKind::Period)), Identifier::parser()).parse(tokens)?;

        Ok((
            tokens,
            Box::new(|expression: Expression<'a>| {
                Fields {
                    expression: Box::new(expression),
                    period,
                    ident,
                }
                .into()
            }),
        ))
    }
}

use super::path::PathExprSegment;
use super::{Expression, NestedExpression};
use crate::template::parser::prelude::*;

/// A field or method.
#[derive(Debug)]
pub(crate) struct Fields<'a> {
    pub(super) expression: Box<Expression<'a>>,
    fields: Vec<Field<'a>>,
}

impl<'a> Fields<'a> {
    /// Parse a field or method.
    pub fn parser(tokens: TokenSlice<'a>) -> Res<'a, Box<NestedExpression<'a>>> {
        let (tokens, fields) = many1(Field::parse).parse(tokens)?;

        Ok((
            tokens,
            Box::new(|expression: Expression<'a>| {
                Self {
                    expression: Box::new(expression),
                    fields,
                }
                .into()
            }),
        ))
    }

    /// Source for the entire group, including the parentheses.
    pub fn source(&self) -> Source<'a> {
        let mut source: Source<'a> = self.expression.source();
        for field in &self.fields {
            source = source.merge(
                &field.source(),
                "Field source should be immediately after the rest of the expression",
            );
        }
        source
    }
}

impl<'a> ToTokensWithState<'a> for Fields<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut tokens = TokenStream::new();
        let (expression, estimated_length) = self.expression.to_tokens_with_state(state);
        tokens.append_all(expression);
        for field in &self.fields {
            tokens.append_all(field.to_tokens_with_state(state).0);
        }
        (tokens, estimated_length)
    }
}

impl<'a> From<Fields<'a>> for Expression<'a> {
    fn from(value: Fields<'a>) -> Self {
        Expression::Fields(value)
    }
}

#[derive(Debug)]
pub(crate) struct Field<'a> {
    dot: Source<'a>,
    segment: PathExprSegment<'a>,
}

impl<'a> Field<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (dot, segment)) =
            (take(TokenKind::Period), PathExprSegment::parse).parse(tokens)?;

        Ok((
            tokens,
            Field {
                dot: dot.source().clone(),
                segment,
            },
        ))
    }

    /// Get the `Source` for the field.
    pub(crate) fn source(&self) -> Source<'a> {
        self.dot.clone().merge(
            &self.segment.source(),
            "Field or method name should immediately follow the dot",
        )
    }
}

impl<'a> ToTokensWithState<'a> for Field<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let span = self.dot.span_token();
        let (segment, estimated_length) = self.segment.to_tokens_with_state(state);
        (quote_spanned! {span=> . #segment }, estimated_length)
    }
}

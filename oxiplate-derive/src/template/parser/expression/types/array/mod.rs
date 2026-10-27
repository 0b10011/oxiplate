use super::Type;
use super::no_bounds::TypeNoBounds;
use crate::template::parser::expression::{Expression, expression};
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types/array.html#railroad-ArrayType>
#[derive(Debug)]
pub(crate) struct ArrayType<'a> {
    r#type: Box<Type<'a>>,
    semicolon: Source<'a>,
    expression: Box<Expression<'a>>,
    source: Source<'a>,
}

impl<'a> ArrayType<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/array.html#railroad-ArrayType>
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (open_bracket, r#type, semicolon, expression, close_bracket)) = (
            take(TokenKind::OpenBracket),
            Type::parse,
            into(take(TokenKind::Semicolon)),
            expression(true),
            take(TokenKind::CloseBracket),
        )
            .parse(tokens)?;

        let source = open_bracket
            .source()
            .to_owned()
            .merge(&r#type.source(), "Type should follow `[`")
            .merge(&semicolon, "`;` should follow type")
            .merge(&expression.source(), "Expression should follow `;`")
            .merge(close_bracket.source(), "`]` should follow expression");

        Ok((
            tokens,
            Self {
                r#type: Box::new(r#type),
                semicolon,
                expression: Box::new(expression),
                source,
            },
        ))
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> ToTokensWithState<'a> for ArrayType<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let span = self.semicolon.span_token();
        let semicolon = quote_spanned! {span=> ; };

        let span = self.source.span_token();
        let (expression, _estimated_length) = self.expression.to_tokens_with_state(state);

        let (r#type, _estimated_length) = self.r#type.to_tokens_with_state(state);

        (
            quote_spanned! {span=> [ #r#type #semicolon #expression ] },
            EstimatedLength::new(0),
        )
    }
}

impl<'a> From<ArrayType<'a>> for TypeNoBounds<'a> {
    fn from(value: ArrayType<'a>) -> Self {
        TypeNoBounds::Array(value)
    }
}

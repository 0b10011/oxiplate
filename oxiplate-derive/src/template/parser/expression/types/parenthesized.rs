use super::Type;
use super::no_bounds::TypeNoBounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types.html#railroad-ParenthesizedType>
#[derive(Debug)]
pub(crate) struct ParenthesizedType<'a> {
    r#type: Box<Type<'a>>,
    source: Source<'a>,
}

impl<'a> ParenthesizedType<'a> {
    /// See: <https://doc.rust-lang.org/reference/types.html#railroad-ParenthesizedType>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (open, r#type, close)) = (
            take(TokenKind::OpenParenthese),
            cut("Type expected after `(`", Type::parse),
            cut("`)` expected after type", take(TokenKind::CloseParenthese)),
        )
            .parse(tokens)?;

        let source = open
            .source()
            .clone()
            .merge(&r#type.source(), "Type expected after `(`")
            .merge(close.source(), "`)` expected after type");

        Ok((
            tokens,
            Self {
                r#type: Box::new(r#type),
                source,
            },
        ))
    }

    pub(super) fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> ToTokensWithState<'a> for ParenthesizedType<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let span = self.source.span_token();
        let (ty, _estimated_length) = &self.r#type.to_tokens_with_state(state);

        (quote_spanned! {span=> ( #ty ) }, EstimatedLength::new(0))
    }
}

impl<'a> From<ParenthesizedType<'a>> for TypeNoBounds<'a> {
    fn from(value: ParenthesizedType<'a>) -> Self {
        Self::Parenthesized(value)
    }
}

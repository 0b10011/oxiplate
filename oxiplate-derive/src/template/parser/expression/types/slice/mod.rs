use super::Type;
use super::no_bounds::TypeNoBounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types/slice.html#railroad-SliceType>
#[derive(Debug)]
pub(crate) struct SliceType<'a> {
    r#type: Box<Type<'a>>,
    source: Source<'a>,
}

impl<'a> SliceType<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/slice.html#railroad-SliceType>
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (open_bracket, r#type, close_bracket)) = (
            take(TokenKind::OpenBracket),
            Type::parse,
            take(TokenKind::CloseBracket),
        )
            .parse(tokens)?;

        let source = open_bracket
            .source()
            .to_owned()
            .merge(&r#type.source(), "Type should follow `[`")
            .merge(close_bracket.source(), "`]` should follow expression");

        Ok((
            tokens,
            Self {
                r#type: Box::new(r#type),
                source,
            },
        ))
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> ToTokensWithState<'a> for SliceType<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let span = self.source.span_token();
        let (r#type, _estimated_length) = self.r#type.to_tokens_with_state(state);

        (
            quote_spanned! {span=> [ #r#type ] },
            EstimatedLength::new(0),
        )
    }
}

impl<'a> From<SliceType<'a>> for TypeNoBounds<'a> {
    fn from(value: SliceType<'a>) -> Self {
        TypeNoBounds::Slice(value)
    }
}

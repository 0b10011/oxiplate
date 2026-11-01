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
    pub(in crate::template::parser) fn parser() -> SliceTypeParser<'a> {
        SliceTypeParser {
            phantom_data: PhantomData,
        }
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

pub(in crate::template::parser) struct SliceTypeParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for SliceTypeParser<'a> {
    type TokenKind = TokenKind;
    type Output = SliceType<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (open_bracket, r#type, close_bracket)) = (
            take(TokenKind::OpenBracket),
            Type::parser(),
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
            SliceType {
                r#type: Box::new(r#type),
                source,
            },
        ))
    }
}

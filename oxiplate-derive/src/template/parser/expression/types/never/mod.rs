use super::no_bounds::TypeNoBounds;
use crate::template::parser::prelude::*;

/// `!`
///
/// See: <https://doc.rust-lang.org/reference/types/never.html#railroad-NeverType>
#[derive(Debug)]
pub(crate) struct Never<'a> {
    source: Source<'a>,
}

impl<'a> Never<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/never.html#railroad-NeverType>
    pub(in crate::template::parser) fn parser() -> NeverParser<'a> {
        NeverParser {
            phantom_data: PhantomData,
        }
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> ToTokensWithState<'a> for Never<'a> {
    fn to_tokens_with_state(&self, _state: &State<'a>) -> BuiltTokens {
        let span = self.source.span_token();
        (quote_spanned! {span=> ! }, EstimatedLength::new(0))
    }
}

impl<'a> From<Never<'a>> for TypeNoBounds<'a> {
    fn from(value: Never<'a>) -> Self {
        TypeNoBounds::Never(value)
    }
}

pub(in crate::template::parser) struct NeverParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for NeverParser<'a> {
    type TokenKind = TokenKind;
    type Output = Never<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, source) = into(take(TokenKind::Exclamation)).parse(tokens)?;

        Ok((tokens, Never { source }))
    }
}

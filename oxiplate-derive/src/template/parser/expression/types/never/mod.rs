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
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, source) = into(take(TokenKind::Exclamation)).parse(tokens)?;

        Ok((tokens, Self { source }))
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

use super::no_bounds::NoBounds;
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

impl ToTokens for Never<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = self.source.span_token();
        tokens.append_all(quote_spanned! {span=> ! });
    }
}

impl<'a> From<Never<'a>> for NoBounds<'a> {
    fn from(value: Never<'a>) -> Self {
        NoBounds::Never(value)
    }
}

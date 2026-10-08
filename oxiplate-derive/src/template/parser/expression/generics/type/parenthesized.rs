use super::Type;
use super::no_bounds::NoBounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types.html#railroad-ParenthesizedType>
#[derive(Debug)]
pub(crate) struct Parenthesized<'a> {
    r#type: Box<Type<'a>>,
    source: Source<'a>,
}

impl<'a> Parenthesized<'a> {
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

impl ToTokens for Parenthesized<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = self.source.span_token();
        let ty = &self.r#type;
        tokens.append_all(quote_spanned! {span=> ( #ty ) });
    }
}

impl<'a> From<Parenthesized<'a>> for NoBounds<'a> {
    fn from(value: Parenthesized<'a>) -> Self {
        Self::Parenthesized(value)
    }
}

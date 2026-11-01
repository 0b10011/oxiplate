use proc_macro2::Literal;

use super::LiteralExpression;
use crate::template::parser::expression::{Expression, Res};
use crate::template::parser::prelude::*;

#[derive(Debug)]
pub(crate) struct String<'a> {
    value: std::string::String,
    source: &'a Source<'a>,
}

impl<'a> String<'a> {
    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-STRING_LITERAL>
    pub(in crate::template::parser) fn parser() -> StringParser<'a> {
        StringParser {
            phantom_data: PhantomData,
        }
    }

    pub(crate) fn as_str(&'a self) -> &'a str {
        &self.value
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        self.source
    }
}

impl<'a> ToTokensWithState<'a> for String<'a> {
    fn to_tokens_with_state(&self, _state: &State<'a>) -> BuiltTokens {
        let mut literal = Literal::string(&self.value);
        literal.set_span(self.source.span_token());
        (
            quote! { #literal },
            EstimatedLength::new(self.value.as_str().len()),
        )
    }
}

impl<'a> From<String<'a>> for Expression<'a> {
    fn from(value: String<'a>) -> Self {
        Expression::String(value)
    }
}

impl<'a> From<String<'a>> for LiteralExpression<'a> {
    fn from(value: String<'a>) -> Self {
        Self::String(value)
    }
}

/// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-STRING_LITERAL>
pub(in crate::template::parser) struct StringParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for StringParser<'a> {
    type TokenKind = TokenKind;
    type Output = String<'a>;

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-STRING_LITERAL>
    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, token) = tokens.take()?;

        let (TokenKind::String(value) | TokenKind::RawString(value)) = token.kind() else {
            return Err(Error::Recoverable {
                message: "Expected a string or raw string".to_string(),
                source: token.source().clone(),
                previous_error: None,
                is_eof: false,
            });
        };

        Ok((
            tokens,
            String {
                value: *value.clone(),
                source: token.source(),
            },
        ))
    }
}

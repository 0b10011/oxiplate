use quote::quote;

use crate::syntax::Error;
use crate::syntax::expression::{Expression, Res};
use crate::tokenizer::parser::{TokenKind, TokenSlice};
use crate::{BuiltTokens, Source};

#[derive(Debug)]
pub(crate) struct String<'a> {
    value: std::string::String,
    source: &'a Source<'a>,
}

impl<'a> String<'a> {
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
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
            Self {
                value: *value.clone(),
                source: token.source(),
            },
        ))
    }

    pub(crate) fn as_str(&'a self) -> &'a str {
        &self.value
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        self.source
    }

    pub(crate) fn to_tokens(&self) -> BuiltTokens {
        let literal = ::syn::LitStr::new(&self.value, self.source.span_token());
        (quote! { #literal }, self.value.as_str().len())
    }
}

impl<'a> From<String<'a>> for Expression<'a> {
    fn from(value: String<'a>) -> Self {
        Expression::String(value)
    }
}

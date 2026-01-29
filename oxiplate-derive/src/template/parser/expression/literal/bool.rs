use quote::quote;

use crate::template::parser::Error;
use crate::template::parser::expression::{Expression, Res};
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{BuiltTokens, Source};

#[derive(Debug)]
pub(crate) struct Bool<'a> {
    value: bool,
    source: &'a Source<'a>,
}

impl<'a> Bool<'a> {
    /// Parses a bool value: `true` or `false`
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, token) = tokens.take()?;

        let TokenKind::Bool(value) = token.kind() else {
            return Err(Error::Recoverable {
                message: "Not `Bool` token kind".to_string(),
                source: token.source().clone(),
                previous_error: None,
                is_eof: false,
            });
        };

        Ok((
            tokens,
            Self {
                value: *value,
                source: token.source(),
            },
        ))
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        self.source
    }

    pub(crate) fn to_tokens(&self) -> BuiltTokens {
        let literal = ::syn::LitBool::new(self.value, self.source.span_token());
        (quote! { #literal }, 0)
    }
}

impl<'a> From<Bool<'a>> for Expression<'a> {
    fn from(value: Bool<'a>) -> Self {
        Expression::Bool(value)
    }
}

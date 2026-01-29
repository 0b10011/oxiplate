use quote::quote;

use crate::template::parser::Error;
use crate::template::parser::expression::{Expression, Res};
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{BuiltTokens, Source};

#[derive(Debug)]
pub(crate) struct Char<'a> {
    value: char,
    source: &'a Source<'a>,
}

impl<'a> Char<'a> {
    /// Parse char literal (e.g., `'a'`).
    /// See: <https://doc.rust-lang.org/reference/tokens.html#character-literals>
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, token) = tokens.take()?;

        let TokenKind::Char(char) = token.kind() else {
            return Err(Error::Recoverable {
                message: "Not `Char` token kind".to_string(),
                source: token.source().clone(),
                previous_error: None,
                is_eof: false,
            });
        };

        Ok((
            tokens,
            Self {
                value: *char,
                source: token.source(),
            },
        ))
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        self.source
    }

    pub(crate) fn to_tokens(&self) -> BuiltTokens {
        let literal = ::syn::LitChar::new(self.value, self.source.span_token());
        (quote! { #literal }, 1)
    }
}

impl<'a> From<Char<'a>> for Expression<'a> {
    fn from(value: Char<'a>) -> Self {
        Expression::Char(value)
    }
}

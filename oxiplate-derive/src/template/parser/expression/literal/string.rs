use proc_macro2::Literal;

use crate::template::parser::expression::{Expression, Res};
use crate::template::parser::prelude::*;

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

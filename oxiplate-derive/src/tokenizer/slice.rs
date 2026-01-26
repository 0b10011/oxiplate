use super::Token;
use crate::Source;
use crate::syntax::UnexpectedTokenError;
use crate::tokenizer::{Eof, TokenKind};

#[derive(Clone, Debug)]
pub struct TokenSlice<'a>(&'a [Token<'a>], &'a Eof<'a>);

impl<'a> TokenSlice<'a> {
    pub fn new(tokens: &'a [Token<'a>], eof: &'a Eof<'a>) -> Self {
        Self(tokens, eof)
    }

    /// Returns first `Token` and remaining tokens.
    pub fn take(self) -> Result<(Self, &'a Token<'a>), UnexpectedTokenError<'a>> {
        self.0
            .split_first()
            .map(|(token, tokens)| {
                if let TokenKind::Unexpected(parse_error) = token.kind() {
                    Err(UnexpectedTokenError::new(
                        parse_error.message(),
                        token.source().clone(),
                        false,
                    ))
                } else {
                    Ok((Self(tokens, self.1), token))
                }
            })
            .unwrap_or(Err(UnexpectedTokenError::new(
                "End of file encountered",
                self.1.source().clone(),
                true,
            )))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn next_source(&self) -> &Source<'a> {
        if let Some(token) = self.0.first() {
            token.source()
        } else {
            self.1.source()
        }
    }

    pub fn eof(&self) -> &'a Eof<'a> {
        self.1
    }
}

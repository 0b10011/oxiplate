use std::fmt::Debug;

use super::{Eof, Token, Tokens};
use crate::tokenizer::UnexpectedTokenError;

#[derive(Debug)]
pub struct TokenSlice<'a, K: Debug + PartialEq + Eq>(
    &'a [Result<Token<'a, K>, UnexpectedTokenError<'a>>],
    &'a Eof<'a>,
);

impl<K: Debug + PartialEq + Eq> Clone for TokenSlice<'_, K> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<'a, K: Debug + PartialEq + Eq> TokenSlice<'a, K> {
    pub fn new(tokens: Tokens<'a, K>, eof: &'a Eof<'a>) -> Self {
        Self(tokens, eof)
    }

    /// Returns first `Token` and remaining tokens.
    pub fn take(self) -> Result<(Self, &'a Token<'a, K>), UnexpectedTokenError<'a>> {
        match self.0.split_first() {
            Some((Ok(token), tokens)) => Ok((Self(tokens, self.1), token)),
            Some((Err(err), _tokens)) => Err(err.clone()),
            None => Err(UnexpectedTokenError::eof(self.1.source().clone())),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn eof(&self) -> &'a Eof<'a> {
        self.1
    }
}

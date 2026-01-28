use super::{Eof, Tokens};
use crate::syntax::UnexpectedTokenError;

#[derive(Debug)]
pub struct TokenSlice<'a, T>(&'a [Result<T, UnexpectedTokenError<'a>>], &'a Eof<'a>);

impl<T> Clone for TokenSlice<'_, T> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<'a, T> TokenSlice<'a, T> {
    pub fn new(tokens: Tokens<'a, T>, eof: &'a Eof<'a>) -> Self {
        Self(tokens, eof)
    }

    /// Returns first `Token` and remaining tokens.
    pub fn take(self) -> Result<(Self, &'a T), UnexpectedTokenError<'a>> {
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

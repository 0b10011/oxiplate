mod buffered_source;
mod slice;
mod token;

use std::fmt::Debug;

pub use self::buffered_source::BufferedSource;
pub use self::slice::TokenSlice;
pub use self::token::Token;
pub use crate::tokenizer::token::ParseError;
use crate::Source;

pub(super) type Tokens<'a, K> = &'a [Result<Token<'a, K>, UnexpectedTokenError<'a>>];

#[derive(Debug)]
pub struct Eof<'a> {
    source: Source<'a>,
}

impl<'a> Eof<'a> {
    #[cfg(test)]
    pub fn for_test(source: Source<'a>) -> Self {
        Self { source }
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }
}

#[derive(Clone, Debug)]
pub struct UnexpectedTokenError<'a> {
    message: &'a str,
    source: Source<'a>,
    is_eof: bool,
}

impl<'a> UnexpectedTokenError<'a> {
    pub fn new(message: &'a str, source: Source<'a>) -> Self {
        Self {
            message,
            source,
            is_eof: false,
        }
    }

    pub fn eof(source: Source<'a>) -> Self {
        Self {
            message: "End of file encountered",
            source,
            is_eof: true,
        }
    }

    pub fn message(&self) -> &'a str {
        self.message
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn is_eof(&self) -> bool {
        self.is_eof
    }
}

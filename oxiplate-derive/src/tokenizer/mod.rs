mod buffered_source;
pub mod parser;
mod slice;
mod token;

use std::fmt::Debug;

use crate::Source;
use crate::syntax::UnexpectedTokenError;
use crate::tokenizer::parser::whitespace;
use crate::tokenizer::token::ParseError;

pub(super) type Tokens<'a, T> = &'a [Result<T, UnexpectedTokenError<'a>>];

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

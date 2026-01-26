use std::marker::PhantomData;

use crate::syntax::parser::Parser;
use crate::syntax::{Error, Res};
use crate::tokenizer::TokenSlice;

/// Builds a parser that always returns an error.
///
/// ```rust,ignore
/// let (tokens, token) = cut(
///     "Parser not expected to be called",
///     fail(),
/// )
/// .parse(tokens)?;
/// ```
pub fn fail<'a, P>() -> Fail<'a, P> {
    Fail {
        phantom_data: PhantomData,
    }
}

pub struct Fail<'a, P> {
    phantom_data: PhantomData<&'a P>,
}

impl<'a, P> Parser<'a> for Fail<'a, P> {
    type Output = P;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        Err(Error::Recoverable {
            message: "`fail()` called".to_string(),
            source: tokens.next_source().clone(),
            previous_error: None,
            is_eof: false,
        })
    }
}

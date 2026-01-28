use std::marker::PhantomData;

use crate::syntax::parser::Parser;
use crate::syntax::{Error, Res};
use crate::tokenizer::parser::TokenSlice;

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
        let source = match tokens.clone().take() {
            Ok((_tokens, token)) => token.source().clone(),
            Err(token_error) => token_error.source().clone(),
        };

        Err(Error::Recoverable {
            message: "`fail()` called".to_string(),
            source,
            previous_error: None,
            is_eof: false,
        })
    }
}

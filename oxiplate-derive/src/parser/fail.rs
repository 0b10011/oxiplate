use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Error, Parser, Res, TokenSlice};

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

impl<'a, K: Debug + PartialEq + Eq, P> Parser<'a, K> for Fail<'a, P> {
    type Output = P;

    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
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

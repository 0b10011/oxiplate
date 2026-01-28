use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Error, Parser, Res, TokenSlice};

/// Builds a parser that turns a recoverable error into an unrecoverable one.
///
/// Wraps recoverable errors with an unrecoverable one
/// that uses the provided error message.
///
/// ```rust,ignore
/// let (tokens, token) = cut(
///     "Expected static text",
///     take(TokenKind::StaticText),
/// )
/// .parse(tokens)?;
/// ```
pub fn cut<'a, K, P>(message: &'static str, parser: P) -> Cut<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    Cut {
        message,
        parser,
        phantom_data: PhantomData,
    }
}

pub struct Cut<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    message: &'static str,
    parser: P,
    phantom_data: PhantomData<&'a K>,
}

impl<'a, K, P> Parser<'a, K> for Cut<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    type Output = <P as Parser<'a, K>>::Output;

    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        match self.parser.parse(tokens) {
            Err(err @ (Error::Recoverable { .. } | Error::Multiple(_))) => {
                Err(Error::Unrecoverable {
                    message: self.message.to_string(),
                    source: err.source().clone(),
                    is_eof: err.is_eof(),
                    previous_error: Some(Box::new(err)),
                })
            }
            result => result,
        }
    }
}

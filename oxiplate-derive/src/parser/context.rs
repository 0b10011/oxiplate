use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Error, Parser, Res, TokenSlice};

/// Builds a parser that adds context to the error, if present.
///
/// ```rust,ignore
/// let (tokens, token) = context(
///     "Attempted to parse static text",
///     take(TokenKind::StaticText),
/// ))
/// .parse(tokens)?;
/// ```
pub fn context<'a, K, P>(message: &'static str, parser: P) -> Context<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    Context {
        message,
        parser,
        phantom_data: PhantomData,
    }
}

pub struct Context<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    message: &'static str,
    parser: P,
    phantom_data: PhantomData<&'a K>,
}

impl<'a, P, K> Parser<'a, K> for Context<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    type Output = <P as Parser<'a, K>>::Output;

    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        match self.parser.parse(tokens) {
            value @ Ok(_) => value,
            Err(err @ Error::Unrecoverable { is_eof, .. }) => Err(Error::Unrecoverable {
                message: self.message.to_string(),
                source: err.source().clone(),
                previous_error: Some(Box::new(err)),
                is_eof,
            }),
            Err(err) => Err(Error::Recoverable {
                message: self.message.to_string(),
                source: err.source().clone(),
                is_eof: err.is_eof(),
                previous_error: Some(Box::new(err)),
            }),
        }
    }
}

use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Parser, Res, TokenSlice};

/// Builds a parser that matches the provided parser
/// while ignoring recoverable errors.
///
/// ```rust,ignore
/// let (tokens, token) = ignore_recoverable_errors(
///     take(TokenKind::StaticText),
/// )
/// .parse(tokens)?;
/// ```
pub fn ignore_recoverable_errors<'a, K, P>(parser: P) -> IgnoreRecoverableErrors<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    IgnoreRecoverableErrors {
        parser,
        phantom_data: PhantomData,
    }
}

pub struct IgnoreRecoverableErrors<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    parser: P,
    phantom_data: PhantomData<&'a K>,
}

impl<'a, K, P> Parser<'a, K> for IgnoreRecoverableErrors<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    type Output = Option<<P as Parser<'a, K>>::Output>;

    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        match self.parser.parse(tokens.clone()) {
            Ok((tokens, output)) => Ok((tokens, Some(output))),
            Err(err) if err.is_recoverable() => Ok((tokens, None)),
            Err(err) => Err(err),
        }
    }
}

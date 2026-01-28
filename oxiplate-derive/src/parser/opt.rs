use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Parser, Res, TokenSlice};

/// Builds a parser that matches the provided parser 0 or 1 times.
///
/// ```rust,ignore
/// let (tokens, token) = opt(
///     take(TokenKind::StaticText),
/// )
/// .parse(tokens)?;
/// ```
pub fn opt<'a, K, P>(parser: P) -> Opt<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    Opt {
        parser,
        phantom_data: PhantomData,
    }
}

pub struct Opt<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    parser: P,
    phantom_data: PhantomData<&'a K>,
}

impl<'a, K, P> Parser<'a, K> for Opt<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    type Output = Option<<P as Parser<'a, K>>::Output>;

    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        match self.parser.parse(tokens.clone()) {
            Ok((tokens, output)) => Ok((tokens, Some(output))),
            Err(_) => Ok((tokens, None)),
        }
    }
}

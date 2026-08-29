use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Parser, Res, TokenSlice};

/// Builds a parser that matches the provided parser
/// while ignoring recoverable and non-recoverable errors.
/// Useful when parsers may conflict with eachother.
///
/// ```rust,ignore
/// let (tokens, token) = ignore_all_errors(
///     cut("Unrecoverable error will be ignored", take(TokenKind::StaticText)),
/// )
/// .parse(tokens)?;
/// ```
pub fn ignore_all_errors<'a, K, P>(parser: P) -> Opt<'a, K, P>
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

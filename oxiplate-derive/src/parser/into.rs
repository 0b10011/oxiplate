use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Parser, Res, TokenSlice};

/// Builds a parser that converts `Ok(F)` to `Ok(T)` via `Into::into()`.
///
/// ```rust,ignore
/// let (tokens, token) = into(
///     take(TokenKind::StaticText),
/// )
/// .parse(tokens)?;
/// ```
pub fn into<'a, K, P, O>(parser: P) -> Into<'a, K, P, O>
where
    O: std::convert::From<<P as Parser<'a, K>>::Output>,
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    Into {
        parser,
        phantom: PhantomData,
        phantom_t: PhantomData,
    }
}

pub struct Into<'a, K, P, O>
where
    O: std::convert::From<<P as Parser<'a, K>>::Output>,
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    parser: P,
    phantom: PhantomData<&'a O>,
    phantom_t: PhantomData<K>,
}

impl<'a, K, P, O> Parser<'a, K> for Into<'a, K, P, O>
where
    O: std::convert::From<<P as Parser<'a, K>>::Output>,
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    type Output = O;

    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        self.parser
            .parse(tokens)
            .map(|(tokens, output)| (tokens, output.into()))
    }
}

use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Parser, Res, TokenSlice};

/// Wraps result of parser in `Some()`.
///
/// ```rust,ignore
/// let (tokens, Some(token)) = some(take(TokenKind::StaticText)).parse(tokens)?;
/// ```
pub(crate) fn some<'a, K, P>(parser: P) -> Some<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    Some {
        parser,
        phantom_data: PhantomData,
    }
}

pub(crate) struct Some<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    parser: P,
    phantom_data: PhantomData<&'a K>,
}

impl<'a, K, P> Parser<'a, K> for Some<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    type Output = Option<<P as Parser<'a, K>>::Output>;

    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        let (tokens, output) = self.parser.parse(tokens.clone())?;

        Ok((tokens, Some(output)))
    }
}

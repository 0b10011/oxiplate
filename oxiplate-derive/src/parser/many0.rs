use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Parser, Res, TokenSlice};

/// Builds a parser that matches the provided parser 0 or more times.
///
/// ```rust,ignore
/// let (tokens, token) = many0(
///     take(TokenKind::StaticText),
/// )
/// .parse(tokens)?;
/// ```
pub fn many0<'a, K, P>(parser: P) -> Many0<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    Many0 {
        parser,
        phantom_data: PhantomData,
    }
}

pub struct Many0<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    parser: P,
    phantom_data: PhantomData<&'a K>,
}

impl<'a, K, P> Parser<'a, K> for Many0<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    type Output = Vec<<P as Parser<'a, K>>::Output>;

    fn parse(&self, mut tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        let mut values = vec![];
        while let Ok((remaining_tokens, output)) = self.parser.parse(tokens.clone()) {
            tokens = remaining_tokens;
            values.push(output);
        }

        Ok((tokens, values))
    }
}

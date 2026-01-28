use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Parser, Res, TokenSlice};

/// Builds a parser that matches the provided parser repeatedly
/// until all tokens are consumed.
///
/// ```rust,ignore
/// let (tokens, token) = parse_all(
///     take(TokenKind::StaticText),
/// )
/// .parse(tokens)?;
/// ```
pub fn parse_all<'a, K, P>(parser: P) -> ParseAll<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    ParseAll {
        parser,
        phantom_data: PhantomData,
    }
}

pub struct ParseAll<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    parser: P,
    phantom_data: PhantomData<&'a K>,
}

impl<'a, K, P> Parser<'a, K> for ParseAll<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    type Output = Vec<<P as Parser<'a, K>>::Output>;

    fn parse(&self, mut tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        let mut values = vec![];
        while !tokens.is_empty() {
            let (remaining_tokens, output) = self.parser.parse(tokens)?;
            tokens = remaining_tokens;
            values.push(output);
        }

        Ok((tokens, values))
    }
}

use std::marker::PhantomData;

use crate::syntax::Res;
use crate::syntax::parser::Parser;
use crate::tokenizer::parser::TokenSlice;

/// Builds a parser that matches the provided parser repeatedly
/// until all tokens are consumed.
///
/// ```rust,ignore
/// let (tokens, token) = parse_all(
///     take(TokenKind::StaticText),
/// )
/// .parse(tokens)?;
/// ```
pub fn parse_all<'a, P>(parser: P) -> ParseAll<'a, P>
where
    P: Parser<'a>,
{
    ParseAll {
        parser,
        phantom_data: PhantomData,
    }
}

pub struct ParseAll<'a, P>
where
    P: Parser<'a>,
{
    parser: P,
    phantom_data: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for ParseAll<'a, P>
where
    P: Parser<'a>,
{
    type Output = Vec<<P as Parser<'a>>::Output>;

    fn parse(&self, mut tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let mut values = vec![];
        while !tokens.is_empty() {
            let (remaining_tokens, output) = self.parser.parse(tokens)?;
            tokens = remaining_tokens;
            values.push(output);
        }

        Ok((tokens, values))
    }
}

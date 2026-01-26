use std::marker::PhantomData;

use crate::syntax::Res;
use crate::syntax::parser::Parser;
use crate::tokenizer::TokenSlice;

/// Builds a parser that matches the provided parser 0 or more times.
///
/// ```rust,ignore
/// let (tokens, token) = many0(
///     take(TokenKind::StaticText),
/// )
/// .parse(tokens)?;
/// ```
pub fn many0<'a, P>(parser: P) -> Many0<'a, P>
where
    P: Parser<'a>,
{
    Many0 {
        parser,
        phantom_data: PhantomData,
    }
}

pub struct Many0<'a, P>
where
    P: Parser<'a>,
{
    parser: P,
    phantom_data: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for Many0<'a, P>
where
    P: Parser<'a>,
{
    type Output = Vec<<P as Parser<'a>>::Output>;

    fn parse(&self, mut tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let mut values = vec![];
        while let Ok((remaining_tokens, output)) = self.parser.parse(tokens.clone()) {
            tokens = remaining_tokens;
            values.push(output);
        }

        Ok((tokens, values))
    }
}

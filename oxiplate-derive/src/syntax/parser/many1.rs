use std::marker::PhantomData;

use crate::syntax::parser::Parser;
use crate::syntax::{Error, Res};
use crate::tokenizer::TokenSlice;

/// Builds a parser that matches the provided parser 1 or more times.
///
/// ```rust,ignore
/// let (tokens, token) = many1(
///     take(TokenKind::StaticText),
/// )
/// .parse(tokens)?;
/// ```
pub fn many1<'a, P>(parser: P) -> Many1<'a, P>
where
    P: Parser<'a>,
{
    Many1 {
        parser,
        phantom_data: PhantomData,
    }
}

pub struct Many1<'a, P>
where
    P: Parser<'a>,
{
    parser: P,
    phantom_data: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for Many1<'a, P>
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

        if values.is_empty() {
            let (_, token) = tokens.clone().take()?;
            return Err(Error::Recoverable {
                message: "Expected to match at least 1".to_string(),
                source: token.source().clone(),
                previous_error: None,
                is_eof: false,
            });
        }

        Ok((tokens, values))
    }
}

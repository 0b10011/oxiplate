use std::marker::PhantomData;

use crate::syntax::Res;
use crate::syntax::parser::Parser;
use crate::tokenizer::TokenSlice;

/// Builds a parser that matches the provided parser 0 or 1 times.
///
/// ```rust,ignore
/// let (tokens, token) = opt(
///     take(TokenKind::StaticText),
/// )
/// .parse(tokens)?;
/// ```
pub fn opt<'a, P>(parser: P) -> Opt<'a, P>
where
    P: Parser<'a>,
{
    Opt {
        parser,
        phantom_data: PhantomData,
    }
}

pub struct Opt<'a, P>
where
    P: Parser<'a>,
{
    parser: P,
    phantom_data: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for Opt<'a, P>
where
    P: Parser<'a>,
{
    type Output = Option<<P as Parser<'a>>::Output>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        match self.parser.parse(tokens.clone()) {
            Ok((tokens, output)) => Ok((tokens, Some(output))),
            Err(_) => Ok((tokens, None)),
        }
    }
}

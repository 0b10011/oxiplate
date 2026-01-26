use std::marker::PhantomData;

use crate::syntax::Res;
use crate::syntax::parser::{Parser, TokenSlice};

/// Builds a parser that converts `Ok(F)` to `Ok(T)` via `Into::into()`.
///
/// ```rust,ignore
/// let (tokens, token) = into(
///     take(TokenKind::StaticText),
/// )
/// .parse(tokens)?;
/// ```
pub fn into<'a, P, O>(parser: P) -> Into<'a, P, O>
where
    O: std::convert::From<<P as Parser<'a>>::Output>,
    P: Parser<'a>,
{
    Into {
        parser,
        phantom: PhantomData,
    }
}

pub struct Into<'a, P, O>
where
    O: std::convert::From<<P as Parser<'a>>::Output>,
    P: Parser<'a>,
{
    parser: P,
    phantom: PhantomData<&'a O>,
}

impl<'a, P, O> Parser<'a> for Into<'a, P, O>
where
    O: std::convert::From<<P as Parser<'a>>::Output>,
    P: Parser<'a>,
{
    type Output = O;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        self.parser
            .parse(tokens)
            .map(|(tokens, output)| (tokens, output.into()))
    }
}

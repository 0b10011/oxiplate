use std::marker::PhantomData;

use crate::syntax::parser::Parser;
use crate::syntax::{Error, Res};
use crate::tokenizer::TokenSlice;

/// Builds a parser that turns a recoverable error into an unrecoverable one.
///
/// Wraps recoverable errors with an unrecoverable one
/// that uses the provided error message.
///
/// ```rust,ignore
/// let (tokens, token) = cut(
///     "Expected static text",
///     take(TokenKind::StaticText),
/// )
/// .parse(tokens)?;
/// ```
pub fn cut<'a, P>(message: &'static str, parser: P) -> Cut<'a, P>
where
    P: Parser<'a>,
{
    Cut {
        message,
        parser,
        phantom_data: PhantomData,
    }
}

pub struct Cut<'a, P>
where
    P: Parser<'a>,
{
    message: &'static str,
    parser: P,
    phantom_data: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for Cut<'a, P>
where
    P: Parser<'a>,
{
    type Output = <P as Parser<'a>>::Output;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        match self.parser.parse(tokens) {
            Err(err @ (Error::Recoverable { .. } | Error::Multiple(_))) => {
                Err(Error::Unrecoverable {
                    message: self.message.to_string(),
                    source: err.source().clone(),
                    is_eof: err.is_eof(),
                    previous_error: Some(Box::new(err)),
                })
            }
            result => result,
        }
    }
}

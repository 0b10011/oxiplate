use std::marker::PhantomData;

use crate::syntax::parser::Parser;
use crate::syntax::{Error, Res};
use crate::tokenizer::parser::TokenSlice;

/// Builds a parser that adds context to the error, if present.
///
/// ```rust,ignore
/// let (tokens, token) = context(
///     "Attempted to parse static text",
///     take(TokenKind::StaticText),
/// ))
/// .parse(tokens)?;
/// ```
pub fn context<'a, P>(message: &'static str, parser: P) -> Context<'a, P>
where
    P: Parser<'a>,
{
    Context {
        message,
        parser,
        phantom_data: PhantomData,
    }
}

pub struct Context<'a, P>
where
    P: Parser<'a>,
{
    message: &'static str,
    parser: P,
    phantom_data: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for Context<'a, P>
where
    P: Parser<'a>,
{
    type Output = <P as Parser<'a>>::Output;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        match self.parser.parse(tokens) {
            value @ Ok(_) => value,
            Err(err @ Error::Unrecoverable { is_eof, .. }) => Err(Error::Unrecoverable {
                message: self.message.to_string(),
                source: err.source().clone(),
                previous_error: Some(Box::new(err)),
                is_eof,
            }),
            Err(err) => Err(Error::Recoverable {
                message: self.message.to_string(),
                source: err.source().clone(),
                is_eof: err.is_eof(),
                previous_error: Some(Box::new(err)),
            }),
        }
    }
}

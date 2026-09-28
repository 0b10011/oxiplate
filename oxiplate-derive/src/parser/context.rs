use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Error, Parser, Res, TokenSlice};

/// Builds a parser that adds context to the error, if present.
///
/// ```rust,ignore
/// let (tokens, token) = context(
///     "Attempted to parse static text",
///     take(TokenKind::StaticText),
/// ))
/// .parse(tokens)?;
/// ```
pub fn context<'a, K, P>(message: &'static str, parser: P) -> Context<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    Context {
        message,
        parser,
        phantom_data: PhantomData,
    }
}

pub struct Context<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    message: &'static str,
    parser: P,
    phantom_data: PhantomData<&'a K>,
}

impl<'a, P, K> Parser<'a, K> for Context<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    type Output = <P as Parser<'a, K>>::Output;

    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        match self.parser.parse(tokens) {
            value @ Ok(_) => value,
            Err(err) if err.is_recoverable() => Err(Error::Recoverable {
                message: self.message.to_string(),
                source: err.source().clone(),
                is_eof: err.is_eof(),
                previous_error: Some(Box::new(err)),
            }),
            Err(err) => Err(Error::Unrecoverable {
                message: self.message.to_string(),
                source: err.source().clone(),
                is_eof: err.is_eof(),
                previous_error: Some(Box::new(err)),
            }),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use std::assert_matches;

    use crate::parser::{Error, Parser as _, cut, take};
    use crate::source::test_source;
    use crate::template::TokenKind;
    use crate::tokenizer::{Eof, Token, TokenSlice};

    #[test]
    fn ok() {
        test_source!(source = "&");

        let eof = Eof::for_test(source.clone());
        let tokens = [Ok(Token::new(TokenKind::Ampersand, &source, None))];
        super::context("Context", take(TokenKind::Ampersand))
            .parse(TokenSlice::new(&tokens, &eof))
            .expect("Ok expected");
    }

    #[test]
    fn recoverable() {
        test_source!(source = "Hello world");

        let eof = Eof::for_test(source.clone());
        let tokens = [Ok(Token::new(TokenKind::StaticText, &source, None))];
        let error = super::context("Context", take(TokenKind::Ampersand))
            .parse(TokenSlice::new(&tokens, &eof))
            .expect_err("Error expected");

        assert_matches!(
            error,
            Error::Recoverable {
                message,
                source: _,
                previous_error: Some(_),
                is_eof: false
            } if &message == "Context"
        );
    }

    #[test]
    fn unrecoverable() {
        test_source!(source = "Hello world");

        let eof = Eof::for_test(source.clone());
        let tokens = [Ok(Token::new(TokenKind::StaticText, &source, None))];
        let error = super::context(
            "Additional context",
            cut("Expected `&`", take(TokenKind::Ampersand)),
        )
        .parse(TokenSlice::new(&tokens, &eof))
        .expect_err("Error expected");

        assert_matches!(
            error,
            Error::Unrecoverable {
                message,
                source: _,
                previous_error: Some(_),
                is_eof: false
            } if &message == "Additional context"
        );
    }
}

use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Error, Parser, Res, TokenSlice};

/// Builds a parser that matches the provided parser 1 or more times.
///
/// ```rust,ignore
/// let (tokens, token) = many1(
///     take(TokenKind::StaticText),
/// )
/// .parse(tokens)?;
/// ```
pub(crate) fn many1<'a, K, P>(parser: P) -> Many1<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    Many1 {
        parser,
        phantom_data: PhantomData,
    }
}

pub(crate) struct Many1<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    parser: P,
    phantom_data: PhantomData<&'a K>,
}

impl<'a, K, P> Parser<'a, K> for Many1<'a, K, P>
where
    K: Debug + PartialEq + Eq,
    P: Parser<'a, K>,
{
    type Output = Vec<<P as Parser<'a, K>>::Output>;

    fn parse(&self, mut tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        let mut values = vec![];
        loop {
            match self.parser.parse(tokens.clone()) {
                Ok((remaining_tokens, output)) => {
                    tokens = remaining_tokens;
                    values.push(output);
                }
                Err(err) if err.is_recoverable() => break,
                Err(err) => return Err(err),
            }
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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    #[test]
    #[should_panic = "Expected static text"]
    fn test_with_cut() {
        use crate::parser::{Parser as _, cut, take};
        use crate::source::test_source;
        use crate::template::TokenKind;
        use crate::tokenizer::{Eof, Token, TokenSlice};

        test_source!(source = "Hello world");
        test_source!(source2 = "&");

        super::many1(cut("Expected static text", take(TokenKind::StaticText)))
            .parse(TokenSlice::new(
                &[
                    Ok(Token::new(TokenKind::StaticText, &source, None)),
                    Ok(Token::new(TokenKind::Ampersand, &source2, None)),
                ],
                &Eof::for_test(source),
            ))
            .unwrap();
    }
}

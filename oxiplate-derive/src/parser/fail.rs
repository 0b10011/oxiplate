use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Error, Parser, Res, TokenSlice};

/// Builds a parser that always returns an error.
///
/// ```rust,ignore
/// let (tokens, token) = cut(
///     "Parser not expected to be called",
///     fail(),
/// )
/// .parse(tokens)?;
/// ```
#[allow(dead_code)]
pub fn fail<'a, P>() -> Fail<'a, P> {
    Fail {
        phantom_data: PhantomData,
    }
}

pub struct Fail<'a, P> {
    phantom_data: PhantomData<&'a P>,
}

impl<'a, K: Debug + PartialEq + Eq, P> Parser<'a, K> for Fail<'a, P> {
    type Output = P;

    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        let source = match tokens.clone().take() {
            Ok((_tokens, token)) => token.source().clone(),
            Err(token_error) => token_error.source().clone(),
        };

        Err(Error::Recoverable {
            message: "`fail()` called".to_string(),
            source,
            previous_error: None,
            is_eof: false,
        })
    }
}

#[test]
#[should_panic = "`fail()` called"]
fn test_error_failure() {
    use super::take;
    use crate::source::test_source;
    use crate::template::TokenKind;
    use crate::tokenizer::{Eof, Token};

    test_source!(source = "Hello world");

    (take(TokenKind::StaticText), fail::<String>())
        .parse(TokenSlice::new(
            &[Ok(Token::new(TokenKind::StaticText, &source, None))],
            &Eof::for_test(source),
        ))
        .unwrap();
}

#[test]
#[should_panic = "`fail()` called"]
fn test_success_failure() {
    use super::take;
    use crate::source::test_source;
    use crate::template::TokenKind;
    use crate::tokenizer::{Eof, Token};

    test_source!(source = "Hello world");
    test_source!(source2 = "Goodbye world");

    (take(TokenKind::StaticText), fail::<String>())
        .parse(TokenSlice::new(
            &[
                Ok(Token::new(TokenKind::StaticText, &source, None)),
                Ok(Token::new(TokenKind::StaticText, &source2, None)),
            ],
            &Eof::for_test(source),
        ))
        .unwrap();
}

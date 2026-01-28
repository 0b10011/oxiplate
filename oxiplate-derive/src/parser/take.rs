use std::fmt::Debug;
use std::marker::PhantomData;

use crate::parser::{Error, Parser, Res, TokenSlice};
use crate::tokenizer::Token;

/// Builds a parser that matches the next token's kind.
///
/// ```rust,ignore
/// let (tokens, token) = take(TokenKind::StaticText).parse(tokens)?;
/// ```
#[inline]
pub fn take<'a, K: Debug + PartialEq + Eq>(expected_token_kind: K) -> Take<'a, K> {
    Take {
        expected_token_kind,
        phantom_data: PhantomData,
    }
}

pub struct Take<'a, K> {
    expected_token_kind: K,
    phantom_data: PhantomData<&'a ()>,
}

impl<'a, K: Debug + PartialEq + Eq + 'a> Parser<'a, K> for Take<'a, K> {
    type Output = &'a Token<'a, K>;

    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        let (tokens, token) = tokens.take()?;

        if *token.kind() == self.expected_token_kind {
            Ok((tokens, token))
        } else {
            Err(Error::Recoverable {
                message: format!(
                    "Expected token kind `{:?}`, found `{:?}`",
                    self.expected_token_kind,
                    token.kind()
                ),
                source: token.source().clone(),
                previous_error: None,
                is_eof: false,
            })
        }
    }
}

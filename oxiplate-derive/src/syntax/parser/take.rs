use std::marker::PhantomData;

use crate::syntax::parser::Parser;
use crate::syntax::{Error, Res};
use crate::tokenizer::parser::{Token, TokenKind, TokenSlice};

/// Builds a parser that matches the next token's kind.
///
/// ```rust,ignore
/// let (tokens, token) = take(TokenKind::StaticText).parse(tokens)?;
/// ```
#[inline]
pub fn take<'a>(expected_token_kind: TokenKind) -> Take<'a> {
    Take {
        expected_token_kind,
        phantom_data: PhantomData,
    }
}

pub struct Take<'a> {
    expected_token_kind: TokenKind,
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for Take<'a> {
    type Output = &'a Token<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
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

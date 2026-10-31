use std::fmt::Debug;
use std::marker::PhantomData;

use super::{Parser, Res, TokenSlice};

/// Generates `None` without consuming any tokens.
///
/// ```rust,ignore
/// let (tokens, (optional_static_text, static_whitespace)) = alt((
///     (some(take(TokenKind::StaticText)), take(TokenKind::StaticWhitespace)),
///     (none(), take(TokenKind::StaticWhitespace))
/// )).parse(tokens)?;
/// ```
pub(crate) fn none<K, P>() -> None<K, P> {
    None {
        phantom_kind: PhantomData,
        phantom_output: PhantomData,
    }
}

pub(crate) struct None<K, P> {
    phantom_kind: PhantomData<K>,
    phantom_output: PhantomData<P>,
}

impl<'a, K, P> Parser<'a> for None<K, P>
where
    K: Debug + PartialEq + Eq,
{
    type TokenKind = K;
    type Output = Option<P>;

    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        Ok((tokens, Option::None))
    }
}

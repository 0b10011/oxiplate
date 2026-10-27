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
pub(crate) fn none<P>() -> None<P> {
    None {
        phantom_data: PhantomData,
    }
}

pub(crate) struct None<P> {
    phantom_data: PhantomData<P>,
}

impl<'a, K, P> Parser<'a, K> for None<P>
where
    K: Debug + PartialEq + Eq,
{
    type Output = Option<P>;

    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
        Ok((tokens, Option::None))
    }
}

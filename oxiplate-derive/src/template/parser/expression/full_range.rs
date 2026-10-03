use super::Expression;
use crate::template::parser::prelude::*;

/// `..` that represents a range
/// where the start/end matches whatever it is applied to.
/// See: <https://doc.rust-lang.org/core/ops/struct.RangeFull.html>
#[derive(Debug)]
pub(crate) struct FullRange<'a> {
    source: Source<'a>,
}

impl<'a> FullRange<'a> {
    /// Helper function to simplify testing.
    #[cfg(test)]
    pub(super) fn new_for_test(source: Source<'a>) -> Self {
        Self { source }
    }

    /// Parses a full range expression (`..`).
    /// See: <https://doc.rust-lang.org/core/ops/struct.RangeFull.html>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, token) = take(TokenKind::RangeExclusive).parse(tokens)?;

        Ok((
            tokens,
            Self {
                source: token.source().clone(),
            },
        ))
    }

    /// Generates token stream for entire expression.
    pub(super) fn to_tokens(&self, _state: &State) -> BuiltTokens {
        let span = self.source.span_token();
        (quote_spanned! {span=> .. }, EstimatedLength::new(0))
    }

    /// Builds source for entire expression.
    pub fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> From<FullRange<'a>> for Expression<'a> {
    fn from(value: FullRange<'a>) -> Self {
        Self::FullRange(value)
    }
}

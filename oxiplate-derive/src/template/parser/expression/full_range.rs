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
    pub(in crate::template::parser) fn parser() -> FullRangeParser<'a> {
        FullRangeParser {
            phantom_data: PhantomData,
        }
    }

    /// Builds source for entire expression.
    pub fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> ToTokensWithState<'a> for FullRange<'a> {
    fn to_tokens_with_state(&self, _state: &State<'a>) -> BuiltTokens {
        let span = self.source.span_token();
        (quote_spanned! {span=> .. }, EstimatedLength::new(0))
    }
}

impl<'a> From<FullRange<'a>> for Expression<'a> {
    fn from(value: FullRange<'a>) -> Self {
        Self::FullRange(value)
    }
}

/// Parses a full range expression (`..`).
/// See: <https://doc.rust-lang.org/core/ops/struct.RangeFull.html>
pub(in crate::template::parser) struct FullRangeParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for FullRangeParser<'a> {
    type TokenKind = TokenKind;
    type Output = FullRange<'a>;

    /// Parses a full range expression (`..`).
    /// See: <https://doc.rust-lang.org/core/ops/struct.RangeFull.html>
    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, token) = take(TokenKind::RangeExclusive).parse(tokens)?;

        Ok((
            tokens,
            FullRange {
                source: token.source().clone(),
            },
        ))
    }
}

use super::args::GenericArg;
use super::path_segment::TypePathSegment;
use crate::template::parser::expression::trait_bounds::bounds::Bounds;
use crate::template::parser::prelude::*;

/// See <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBounds>
#[derive(Debug)]
pub(in crate::template::parser) struct GenericArgsBounds<'a> {
    path_segment: TypePathSegment<'a>,
    colon: Source<'a>,
    bounds: Option<Bounds<'a>>,
}

impl<'a> GenericArgsBounds<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBounds>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (path_segment, colon, bounds)) = (
            TypePathSegment::parse,
            take(TokenKind::Colon),
            ignore_recoverable_errors(Bounds::parse),
        )
            .parse(tokens)?;

        let colon = colon.source().clone();

        Ok((
            tokens,
            Self {
                path_segment,
                colon,
                bounds,
            },
        ))
    }

    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
        self.path_segment
            .source()
            .clone()
            .merge(&self.colon, "`:` expected after path segment")
            .merge_some(
                self.bounds.as_ref().map(Bounds::source).as_ref(),
                "Trait bounds expected after `:`",
            )
    }
}

impl<'a> ToTokensWithState<'a> for GenericArgsBounds<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let span = self.colon.span_token();

        let path_segment = self.path_segment.to_tokens_with_state(state).0;
        let tokens = if let Some(bounds) = &self.bounds {
            let bounds = bounds.to_tokens_with_state(state).0;
            quote_spanned! {span=> #path_segment : #bounds }
        } else {
            quote_spanned! {span=> #path_segment : }
        };

        (tokens, EstimatedLength::new(0))
    }
}

impl<'a> From<GenericArgsBounds<'a>> for GenericArg<'a> {
    fn from(value: GenericArgsBounds<'a>) -> Self {
        Self::Bounds(value)
    }
}

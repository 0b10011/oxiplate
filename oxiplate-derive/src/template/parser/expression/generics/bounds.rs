use super::args::GenericArg;
use super::r#type::bounds::Bounds as TraitBounds;
use super::r#type::path_segment::PathSegment;
use crate::template::parser::prelude::*;

/// See <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBounds>
#[derive(Debug)]
pub(super) struct Bounds<'a> {
    path_segment: PathSegment<'a>,
    colon: Source<'a>,

    #[expect(clippy::struct_field_names)]
    bounds: Option<TraitBounds<'a>>,
}

impl<'a> Bounds<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBounds>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (path_segment, colon, bounds)) = (
            PathSegment::parse,
            take(TokenKind::Colon),
            ignore_recoverable_errors(TraitBounds::parse),
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

    pub(super) fn source(&self) -> Source<'a> {
        self.path_segment
            .source()
            .clone()
            .merge(&self.colon, "`:` expected after path segment")
            .merge_some(
                self.bounds.as_ref().map(TraitBounds::source).as_ref(),
                "Trait bounds expected after `:`",
            )
    }
}

impl ToTokens for Bounds<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path_segment.to_tokens(tokens);
        let span = self.colon.span_token();
        tokens.append_all(quote_spanned! {span=> : });
        self.bounds.to_tokens(tokens);
    }
}

impl<'a> From<Bounds<'a>> for GenericArg<'a> {
    fn from(value: Bounds<'a>) -> Self {
        Self::Bounds(value)
    }
}

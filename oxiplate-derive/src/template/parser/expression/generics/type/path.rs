use super::no_bounds::NoBounds;
use super::path_segment::PathSegment;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypePath>
#[derive(Debug)]
pub(crate) struct Path<'a> {
    leading_separator: Option<Source<'a>>,
    first_segment: Box<PathSegment<'a>>,
    additional_segments: Vec<(Source<'a>, PathSegment<'a>)>,
}

impl<'a> Path<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypePath>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (leading_separator, first_segment, additional_segments)) = (
            ignore_recoverable_errors(take(TokenKind::PathSeparator)),
            PathSegment::parse,
            many0((take(TokenKind::PathSeparator), PathSegment::parse)),
        )
            .parse(tokens)?;

        let first_segment = Box::new(first_segment);
        let leading_separator = leading_separator.map(|separator| separator.source().clone());
        let additional_segments = additional_segments
            .into_iter()
            .map(|(separator, segment)| {
                let separator = separator.source().clone();

                (separator, segment)
            })
            .collect();

        Ok((
            tokens,
            Self {
                leading_separator,
                first_segment,
                additional_segments,
            },
        ))
    }

    pub(super) fn source(&self) -> Source<'a> {
        let mut source = self.first_segment.source().clone().append_to_some(
            self.leading_separator.clone(),
            "First segment expected after `::`",
        );

        for (separator, segment) in &self.additional_segments {
            source = source
                .merge(separator, "`::` expected after previous segment")
                .merge(&segment.source(), "Segment expected after `::`");
        }

        source
    }
}

impl ToTokens for Path<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(separator) = &self.leading_separator {
            let span = separator.span_token();
            tokens.append_all(quote_spanned! {span=> :: });
        }

        self.first_segment.to_tokens(tokens);

        for (separator, segment) in &self.additional_segments {
            let span = separator.span_token();
            tokens.append_all(quote_spanned! {span=> :: });

            segment.to_tokens(tokens);
        }
    }
}

impl<'a> From<Path<'a>> for NoBounds<'a> {
    fn from(value: Path<'a>) -> Self {
        Self::Path(value)
    }
}

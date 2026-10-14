use super::path_segment::TypePathSegment;
use crate::template::parser::expression::types::no_bounds::TypeNoBounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypePath>
#[derive(Debug)]
pub(crate) struct TypePath<'a> {
    leading_separator: Option<Source<'a>>,
    first_segment: Box<TypePathSegment<'a>>,
    additional_segments: Vec<(Source<'a>, TypePathSegment<'a>)>,
}

impl<'a> TypePath<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypePath>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (leading_separator, first_segment, additional_segments)) = (
            ignore_recoverable_errors(take(TokenKind::PathSeparator)),
            TypePathSegment::parse,
            many0((take(TokenKind::PathSeparator), TypePathSegment::parse)),
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

    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
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

impl<'a> ToTokensWithState<'a> for TypePath<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let mut tokens = TokenStream::new();

        if let Some(separator) = &self.leading_separator {
            let span = separator.span_token();
            tokens.append_all(quote_spanned! {span=> :: });
        }

        tokens.append_all(self.first_segment.to_tokens_with_state(state).0);

        for (separator, segment) in &self.additional_segments {
            let span = separator.span_token();
            tokens.append_all(quote_spanned! {span=> :: });

            tokens.append_all(segment.to_tokens_with_state(state).0);
        }

        (tokens, EstimatedLength::new(0))
    }
}

impl<'a> From<TypePath<'a>> for TypeNoBounds<'a> {
    fn from(value: TypePath<'a>) -> Self {
        Self::Path(value)
    }
}

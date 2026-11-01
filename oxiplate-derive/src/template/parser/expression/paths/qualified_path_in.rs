use super::path_segment::TypePathSegment;
use super::qualified_path::QualifiedPathType;
use crate::template::parser::expression::types::no_bounds::TypeNoBounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-QualifiedPathInType>
#[derive(Debug)]
pub(crate) struct QualifiedPathInType<'a> {
    qualified_path: QualifiedPathType<'a>,
    type_path_segments: Vec<(Source<'a>, TypePathSegment<'a>)>,
}

impl<'a> QualifiedPathInType<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-QualifiedPathInType>
    pub(in crate::template::parser) fn parser() -> QualifiedPathInTypeParser<'a> {
        QualifiedPathInTypeParser {
            phantom_data: PhantomData,
        }
    }

    pub(crate) fn source(&self) -> Source<'a> {
        let mut source = self.qualified_path.source().to_owned();

        for (separator, type_path_segment) in &self.type_path_segments {
            source = source
                .merge(
                    separator,
                    "`::` expected after qualified path or previous segment",
                )
                .merge(&type_path_segment.source(), "Segment expected after `::`");
        }

        source
    }
}

impl<'a> ToTokensWithState<'a> for QualifiedPathInType<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let qualified_path = self.qualified_path.to_tokens_with_state(state).0;
        let type_path_segments =
            self.type_path_segments
                .iter()
                .map(|(separator, type_path_segment)| {
                    let span = separator.span_token();
                    let type_path_segment = type_path_segment.to_tokens_with_state(state).0;

                    quote_spanned! {span=> :: #type_path_segment }
                });

        (
            quote! { #qualified_path #(#type_path_segments)* },
            EstimatedLength::new(0),
        )
    }
}

impl<'a> From<QualifiedPathInType<'a>> for TypeNoBounds<'a> {
    fn from(value: QualifiedPathInType<'a>) -> Self {
        TypeNoBounds::QualifiedPathIn(value)
    }
}

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-QualifiedPathInType>
pub(in crate::template::parser) struct QualifiedPathInTypeParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for QualifiedPathInTypeParser<'a> {
    type TokenKind = TokenKind;
    type Output = QualifiedPathInType<'a>;

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-QualifiedPathInType>
    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (qualified_path, type_path_segments)) = (
            QualifiedPathType::parser(),
            cut(
                "`::` and type path segment expected after qualified path type",
                many1((
                    into(take(TokenKind::PathSeparator)),
                    TypePathSegment::parser(),
                )),
            ),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            QualifiedPathInType {
                qualified_path,
                type_path_segments,
            },
        ))
    }
}

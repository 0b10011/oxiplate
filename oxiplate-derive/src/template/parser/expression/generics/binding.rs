use super::args::GenericArg;
use super::r#type::Type;
use super::r#type::path_segment::TypePathSegment;
use crate::template::parser::prelude::*;

/// See <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBinding>
#[derive(Debug)]
pub(super) struct Binding<'a> {
    path_segment: TypePathSegment<'a>,
    equals: Source<'a>,
    r#type: Type<'a>,
}

impl<'a> Binding<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBinding>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (path_segment, equals, r#type)) =
            (TypePathSegment::parse, take(TokenKind::Equal), Type::parse).parse(tokens)?;

        let equals = equals.source().clone();

        Ok((
            tokens,
            Self {
                path_segment,
                equals,
                r#type,
            },
        ))
    }

    pub(super) fn source(&self) -> Source<'a> {
        self.path_segment
            .source()
            .clone()
            .merge(&self.equals, "`=` expected after path segment")
            .merge(&self.r#type.source(), "Type expected after `=`")
    }
}

impl<'a> ToTokensWithState<'a> for Binding<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let span = self.equals.span_token();

        let path_segment = self.path_segment.to_tokens_with_state(state).0;
        let r#type = self.r#type.to_tokens_with_state(state).0;

        (
            quote_spanned! {span=> #path_segment = #r#type },
            EstimatedLength::new(0),
        )
    }
}

impl<'a> From<Binding<'a>> for GenericArg<'a> {
    fn from(value: Binding<'a>) -> Self {
        Self::Binding(value)
    }
}

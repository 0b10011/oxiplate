use super::args::GenericArg;
use super::r#type::Type;
use super::r#type::path_segment::PathSegment;
use crate::template::parser::prelude::*;

/// See <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBinding>
#[derive(Debug)]
pub(super) struct Binding<'a> {
    path_segment: PathSegment<'a>,
    equals: Source<'a>,
    r#type: Type<'a>,
}

impl<'a> Binding<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBinding>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (path_segment, equals, r#type)) =
            (PathSegment::parse, take(TokenKind::Equal), Type::parse).parse(tokens)?;

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

impl ToTokens for Binding<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path_segment.to_tokens(tokens);
        let span = self.equals.span_token();
        tokens.append_all(quote_spanned! {span=> = });
        self.r#type.to_tokens(tokens);
    }
}

impl<'a> From<Binding<'a>> for GenericArg<'a> {
    fn from(value: Binding<'a>) -> Self {
        Self::Binding(value)
    }
}

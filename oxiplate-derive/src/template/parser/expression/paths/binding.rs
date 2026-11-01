use super::args::GenericArg;
use super::path_segment::TypePathSegment;
use crate::template::parser::expression::types::Type;
use crate::template::parser::prelude::*;

/// See <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBinding>
#[derive(Debug)]
pub(in crate::template::parser) struct GenericArgsBinding<'a> {
    path_segment: TypePathSegment<'a>,
    equals: Source<'a>,
    r#type: Type<'a>,
}

impl<'a> GenericArgsBinding<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBinding>
    pub(in crate::template::parser) fn parser() -> GenericArgsBindingParser<'a> {
        GenericArgsBindingParser {
            phantom_data: PhantomData,
        }
    }

    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
        self.path_segment
            .source()
            .clone()
            .merge(&self.equals, "`=` expected after path segment")
            .merge(&self.r#type.source(), "Type expected after `=`")
    }
}

impl<'a> ToTokensWithState<'a> for GenericArgsBinding<'a> {
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

impl<'a> From<GenericArgsBinding<'a>> for GenericArg<'a> {
    fn from(value: GenericArgsBinding<'a>) -> Self {
        Self::Binding(value)
    }
}

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBinding>
pub(in crate::template::parser) struct GenericArgsBindingParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for GenericArgsBindingParser<'a> {
    type TokenKind = TokenKind;
    type Output = GenericArgsBinding<'a>;

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgsBinding>
    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (path_segment, equals, r#type)) = (
            TypePathSegment::parser(),
            into(take(TokenKind::Equal)),
            cut("Expected type after `=`", Type::parser()),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            GenericArgsBinding {
                path_segment,
                equals,
                r#type,
            },
        ))
    }
}

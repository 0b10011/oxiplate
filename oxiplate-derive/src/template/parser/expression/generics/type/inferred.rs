use super::no_bounds::TypeNoBounds;
use crate::template::parser::expression::KeywordParser;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types/inferred.html#railroad-InferredType>
#[derive(Debug)]
pub(crate) struct InferredType<'a> {
    source: Source<'a>,
}

impl<'a> InferredType<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/inferred.html#railroad-InferredType>
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, underscore) = KeywordParser::new("_").parse(tokens)?;

        Ok((
            tokens,
            Self {
                source: underscore.source().to_owned(),
            },
        ))
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> ToTokensWithState<'a> for InferredType<'a> {
    fn to_tokens_with_state(&self, _state: &State<'a>) -> BuiltTokens {
        let span = self.source.span_token();

        (quote_spanned! {span=> _ }, EstimatedLength::new(0))
    }
}

impl<'a> From<InferredType<'a>> for TypeNoBounds<'a> {
    fn from(value: InferredType<'a>) -> Self {
        TypeNoBounds::Inferred(value)
    }
}

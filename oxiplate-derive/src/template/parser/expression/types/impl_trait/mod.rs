pub(in crate::template::parser) mod impl_trait_one_bound;

use super::Type;
use crate::template::parser::expression::KeywordParser;
use crate::template::parser::expression::trait_bounds::bounds::Bounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitType>
#[derive(Debug)]
pub(crate) struct ImplTraitType<'a> {
    r#impl: Source<'a>,
    bounds: Option<Bounds<'a>>,
}

impl<'a> ImplTraitType<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitType>
    pub(in crate::template::parser) fn parser() -> ImplTraitTypeParser<'a> {
        ImplTraitTypeParser {
            phantom_data: PhantomData,
        }
    }

    pub(super) fn source(&self) -> Source<'a> {
        self.r#impl.clone().merge_some(
            self.bounds.as_ref().map(Bounds::source).as_ref(),
            "Trait bound expected after `impl`",
        )
    }
}

impl<'a> ToTokensWithState<'a> for ImplTraitType<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let span = self.r#impl.span_token();

        let tokens = if let Some(bounds) = &self.bounds {
            let bounds = bounds.to_tokens_with_state(state).0;
            quote_spanned! {span=> impl #bounds }
        } else {
            quote_spanned! {span=> impl }
        };

        (tokens, EstimatedLength::new(0))
    }
}

impl<'a> From<ImplTraitType<'a>> for Type<'a> {
    fn from(value: ImplTraitType<'a>) -> Self {
        Self::ImplTrait(value)
    }
}

/// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitType>
pub(in crate::template::parser) struct ImplTraitTypeParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for ImplTraitTypeParser<'a> {
    type TokenKind = TokenKind;
    type Output = ImplTraitType<'a>;

    /// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitType>
    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (r#impl, bounds)) = (
            KeywordParser::new("impl"),
            cut(
                "Trait bound expected after `impl`",
                ignore_recoverable_errors(Bounds::parser()),
            ),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            ImplTraitType {
                r#impl: r#impl.source().clone(),
                bounds,
            },
        ))
    }
}

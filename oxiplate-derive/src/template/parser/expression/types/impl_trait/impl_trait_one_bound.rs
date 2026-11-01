use crate::template::parser::expression::KeywordParser;
use crate::template::parser::expression::trait_bounds::trait_bound::TraitBound;
use crate::template::parser::expression::types::no_bounds::TypeNoBounds;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitTypeOneBound>
#[derive(Debug)]
pub(in crate::template::parser) struct ImplTraitTypeOneBound<'a> {
    r#impl: Source<'a>,
    trait_bound: Option<TraitBound<'a>>,
}

impl<'a> ImplTraitTypeOneBound<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitTypeOneBound>
    pub(in crate::template::parser) fn parser() -> ImplTraitTypeOneBoundParser<'a> {
        ImplTraitTypeOneBoundParser {
            phantom_data: PhantomData,
        }
    }

    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
        self.r#impl.clone().merge_some(
            self.trait_bound.as_ref().map(TraitBound::source).as_ref(),
            "Trait bound expected after `impl`",
        )
    }
}

impl<'a> ToTokensWithState<'a> for ImplTraitTypeOneBound<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let span = self.r#impl.span_token();

        let tokens = if let Some(trait_bound) = &self.trait_bound {
            let trait_bound = trait_bound.to_tokens_with_state(state).0;
            quote_spanned! {span=> impl #trait_bound }
        } else {
            quote_spanned! {span=> impl }
        };

        (tokens, EstimatedLength::new(0))
    }
}

impl<'a> From<ImplTraitTypeOneBound<'a>> for TypeNoBounds<'a> {
    fn from(value: ImplTraitTypeOneBound<'a>) -> Self {
        Self::ImplTraitOneBound(value)
    }
}

/// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitTypeOneBound>
pub(in crate::template::parser) struct ImplTraitTypeOneBoundParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for ImplTraitTypeOneBoundParser<'a> {
    type TokenKind = TokenKind;
    type Output = ImplTraitTypeOneBound<'a>;

    /// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitTypeOneBound>
    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (r#impl, trait_bound)) = (
            KeywordParser::new("impl"),
            cut(
                "Trait bound expected after `impl`",
                ignore_recoverable_errors(TraitBound::parser()),
            ),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            ImplTraitTypeOneBound {
                r#impl: r#impl.source().clone(),
                trait_bound,
            },
        ))
    }
}

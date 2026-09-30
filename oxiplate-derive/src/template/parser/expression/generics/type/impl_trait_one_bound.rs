use super::no_bounds::NoBounds;
use super::trait_bound::TraitBound;
use crate::template::parser::expression::KeywordParser;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitTypeOneBound>
#[derive(Debug)]
pub(crate) struct ImplTraitOneBound<'a> {
    r#impl: Source<'a>,
    trait_bound: Option<TraitBound<'a>>,
}

impl<'a> ImplTraitOneBound<'a> {
    /// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitTypeOneBound>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (r#impl, trait_bound)) = (
            KeywordParser::new("impl"),
            cut(
                "Trait bound expected after `impl`",
                ignore_recoverable_errors(TraitBound::parse),
            ),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            Self {
                r#impl: r#impl.source().clone(),
                trait_bound,
            },
        ))
    }

    pub(super) fn source(&self) -> Source<'a> {
        self.r#impl.clone().merge_some(
            self.trait_bound.as_ref().map(TraitBound::source).as_ref(),
            "Trait bound expected after `impl`",
        )
    }
}

impl ToTokens for ImplTraitOneBound<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = self.r#impl.span_token();
        tokens.append_all(quote_spanned! {span=> impl });

        if let Some(trait_bound) = &self.trait_bound {
            tokens.append_all(quote_spanned! {span=> #trait_bound });
        }
    }
}

impl<'a> From<ImplTraitOneBound<'a>> for NoBounds<'a> {
    fn from(value: ImplTraitOneBound<'a>) -> Self {
        Self::ImplTraitOneBound(value)
    }
}

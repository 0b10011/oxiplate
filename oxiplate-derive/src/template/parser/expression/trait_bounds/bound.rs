use super::lifetime::Lifetime;
use super::trait_bound::TraitBound;
use super::use_bound::UseBound;
use crate::template::parser::prelude::*;

/// Between `first_bounds` and `last_bound`,
/// there should always be at least one bound.
///
/// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-Bound>
#[derive(Debug)]
pub(in crate::template::parser) enum Bound<'a> {
    Lifetime(Lifetime<'a>),
    TraitBound(TraitBound<'a>),
    UseBound(UseBound<'a>),
}

impl<'a> Bound<'a> {
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-Bound>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, bound) = alt((
            into(Lifetime::parse),
            into(UseBound::parse),
            into(TraitBound::parse),
        ))
        .parse(tokens)?;

        Ok((tokens, bound))
    }

    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
        match self {
            Self::Lifetime(lifetime) => lifetime.source().clone(),
            Self::TraitBound(trait_bound) => trait_bound.source(),
            Self::UseBound(use_bound) => use_bound.source(),
        }
    }
}

impl<'a> ToTokensWithState<'a> for Bound<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::Lifetime(lifetime) => lifetime.to_tokens_with_state(state),
            Self::TraitBound(trait_bound) => trait_bound.to_tokens_with_state(state),
            Self::UseBound(use_bound) => use_bound.to_tokens_with_state(state),
        }
    }
}

impl<'a> From<Lifetime<'a>> for Bound<'a> {
    fn from(value: Lifetime<'a>) -> Self {
        Self::Lifetime(value)
    }
}

impl<'a> From<TraitBound<'a>> for Bound<'a> {
    fn from(value: TraitBound<'a>) -> Self {
        Self::TraitBound(value)
    }
}

impl<'a> From<UseBound<'a>> for Bound<'a> {
    fn from(value: UseBound<'a>) -> Self {
        Self::UseBound(value)
    }
}

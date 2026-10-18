pub(in crate::template::parser) mod array;
pub(in crate::template::parser) mod impl_trait;
pub(in crate::template::parser) mod inferred;
pub(in crate::template::parser) mod never;
pub(in crate::template::parser) mod no_bounds;
pub(in crate::template::parser) mod parenthesized;
pub(in crate::template::parser) mod pointer;
pub(in crate::template::parser) mod slice;
pub(in crate::template::parser) mod trait_object;
pub(in crate::template::parser) mod tuple;

use self::impl_trait::ImplTraitType;
use self::no_bounds::TypeNoBounds;
use self::trait_object::TraitObjectType;
use super::paths::args::GenericArg;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types.html#railroad-Type>
#[derive(Debug)]
pub(super) enum Type<'a> {
    /// See: <https://doc.rust-lang.org/reference/types.html#railroad-TypeNoBounds>
    NoBounds(TypeNoBounds<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitType>
    ImplTrait(ImplTraitType<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/trait-object.html#railroad-TraitObjectType>
    TraitObject(TraitObjectType<'a>),
}

impl<'a> Type<'a> {
    /// See: <https://doc.rust-lang.org/reference/types.html#railroad-Type>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((
            into(TypeNoBounds::parse),
            into(ImplTraitType::parse),
            into(TraitObjectType::parse),
        ))
        .parse(tokens)
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::NoBounds(no_bounds) => no_bounds.source(),
            Self::ImplTrait(impl_trait_type) => impl_trait_type.source(),
            Self::TraitObject(trait_object_type) => trait_object_type.source(),
        }
    }
}

impl<'a> ToTokensWithState<'a> for Type<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::NoBounds(no_bounds) => no_bounds.to_tokens_with_state(state),
            Self::ImplTrait(impl_trait_type) => impl_trait_type.to_tokens_with_state(state),
            Self::TraitObject(trait_object_type) => trait_object_type.to_tokens_with_state(state),
        }
    }
}

impl<'a> From<Type<'a>> for GenericArg<'a> {
    fn from(value: Type<'a>) -> Self {
        Self::Type(value)
    }
}

mod bound;
pub(crate) mod bounds;
mod impl_trait;
mod impl_trait_one_bound;
mod no_bounds;
mod parenthesized;
mod path;
pub(crate) mod path_segment;
mod trait_bound;
mod trait_bound_inner;
mod use_bound;
mod use_bound_generic_args;

use self::impl_trait::ImplTraitType;
use self::no_bounds::NoBounds;
use super::args::GenericArg;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types.html#railroad-Type>
#[derive(Debug)]
pub(super) enum Type<'a> {
    /// See: <https://doc.rust-lang.org/reference/types.html#railroad-TypeNoBounds>
    NoBounds(NoBounds<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitType>
    ImplTrait(ImplTraitType<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/trait-object.html#railroad-TraitObjectType>
    #[expect(dead_code, reason = "Not yet implemented")]
    TraitObject,
}

impl<'a> Type<'a> {
    /// See: <>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((into(NoBounds::parse), into(ImplTraitType::parse))).parse(tokens)
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::NoBounds(no_bounds) => no_bounds.source(),
            Self::ImplTrait(impl_trait_type) => impl_trait_type.source(),
            Self::TraitObject => todo!("Type::TraitObject.source() not yet implemented"),
        }
    }
}

impl ToTokens for Type<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::NoBounds(no_bounds) => no_bounds.to_tokens(tokens),
            Self::ImplTrait(impl_trait_type) => impl_trait_type.to_tokens(tokens),
            Self::TraitObject => todo!("Type::TraitObject.to_tokens() not yet implemented"),
        }
    }
}

impl<'a> From<Type<'a>> for GenericArg<'a> {
    fn from(value: Type<'a>) -> Self {
        Self::Type(value)
    }
}

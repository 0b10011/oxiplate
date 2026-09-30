use self::no_bounds::NoBounds;
use super::args::GenericArg;
use crate::template::parser::prelude::*;

mod impl_trait_one_bound;
mod no_bounds;
mod parenthesized;
mod path;
mod path_segment;
mod trait_bound;
mod trait_bound_inner;

/// See: <https://doc.rust-lang.org/reference/types.html#railroad-Type>
#[derive(Debug)]
pub(super) enum Type<'a> {
    /// See: <https://doc.rust-lang.org/reference/types.html#railroad-TypeNoBounds>
    NoBounds(NoBounds<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitType>
    #[expect(dead_code, reason = "Not yet implemented")]
    ImplTrait,

    /// See: <https://doc.rust-lang.org/reference/types/trait-object.html#railroad-TraitObjectType>
    #[expect(dead_code, reason = "Not yet implemented")]
    TraitObject,
}

impl<'a> Type<'a> {
    /// See: <>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        into(NoBounds::parse).parse(tokens)
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::NoBounds(no_bounds) => no_bounds.source(),
            Self::ImplTrait => todo!("Type::ImplTrait.source() not yet implemented"),
            Self::TraitObject => todo!("Type::TraitObject.source() not yet implemented"),
        }
    }
}

impl ToTokens for Type<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let _ = tokens;
        todo!("to_tokens() not yet implemented");
    }
}

impl<'a> From<Type<'a>> for GenericArg<'a> {
    fn from(value: Type<'a>) -> Self {
        Self::Type(value)
    }
}

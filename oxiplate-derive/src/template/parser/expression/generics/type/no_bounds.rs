use super::Type;
use super::impl_trait_one_bound::ImplTraitOneBound;
use super::parenthesized::Parenthesized;
use super::path::Path;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types.html#railroad-TypeNoBounds>
#[derive(Debug)]
pub(crate) enum NoBounds<'a> {
    /// See: <https://doc.rust-lang.org/reference/types.html#railroad-ParenthesizedType>
    Parenthesized(Parenthesized<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitTypeOneBound>
    ImplTraitOneBound(ImplTraitOneBound<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/trait-object.html#railroad-TraitObjectTypeOneBound>
    #[expect(dead_code, reason = "Not yet implemented")]
    TraitObjectOneBound,

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypePath>
    Path(Path<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/tuple.html#railroad-TupleType>
    #[expect(dead_code, reason = "Not yet implemented")]
    Tuple,

    /// See: <https://doc.rust-lang.org/reference/types/never.html#railroad-NeverType>
    #[expect(dead_code, reason = "Not yet implemented")]
    Never,

    /// See: <https://doc.rust-lang.org/reference/types/pointer.html#railroad-RawPointerType>
    #[expect(dead_code, reason = "Not yet implemented")]
    RawPointer,

    /// See: <https://doc.rust-lang.org/reference/types/pointer.html#railroad-ReferenceType>
    #[expect(dead_code, reason = "Not yet implemented")]
    Reference,

    /// See: <https://doc.rust-lang.org/reference/types/array.html#railroad-ArrayType>
    #[expect(dead_code, reason = "Not yet implemented")]
    Array,

    /// See: <https://doc.rust-lang.org/reference/types/slice.html#railroad-SliceType>
    #[expect(dead_code, reason = "Not yet implemented")]
    Slice,

    /// See: <https://doc.rust-lang.org/reference/types/inferred.html#railroad-InferredType>
    #[expect(dead_code, reason = "Not yet implemented")]
    Inferred,

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-QualifiedPathInType>
    #[expect(dead_code, reason = "Not yet implemented")]
    QualifiedPathIn,

    /// See: <https://doc.rust-lang.org/reference/types/function-pointer.html#railroad-BareFunctionType>
    #[expect(dead_code, reason = "Not yet implemented")]
    BareFunction,

    /// See: <https://doc.rust-lang.org/reference/macros.html#railroad-MacroInvocation>
    #[expect(dead_code, reason = "Not yet implemented")]
    MacroInvocation,
}

impl<'a> NoBounds<'a> {
    /// See: <https://doc.rust-lang.org/reference/types.html#railroad-TypeNoBounds>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((into(Parenthesized::parse), into(ImplTraitOneBound::parse))).parse(tokens)
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::Parenthesized(parenthesized) => parenthesized.source().clone(),
            Self::ImplTraitOneBound(impl_trait_one_bound) => impl_trait_one_bound.source(),
            Self::TraitObjectOneBound => {
                todo!("NoBounds::TraitObjectOneBound.source() not yet implemented")
            }
            Self::Path(type_path) => type_path.source().clone(),
            Self::Tuple => todo!("NoBounds::Tuple.source() not yet implemented"),
            Self::Never => todo!("NoBounds::Never.source() not yet implemented"),
            Self::RawPointer => todo!("NoBounds::RawPointer.source() not yet implemented"),
            Self::Reference => todo!("NoBounds::Reference.source() not yet implemented"),
            Self::Array => todo!("NoBounds::Array.source() not yet implemented"),
            Self::Slice => todo!("NoBounds::Slice.source() not yet implemented"),
            Self::Inferred => todo!("NoBounds::Inferred.source() not yet implemented"),
            Self::QualifiedPathIn => {
                todo!("NoBounds::QualifiedPathIn.source() not yet implemented")
            }
            Self::BareFunction => todo!("NoBounds::BareFunction.source() not yet implemented"),
            Self::MacroInvocation => {
                todo!("NoBounds::MacroInvocation.source() not yet implemented")
            }
        }
    }
}

impl ToTokens for NoBounds<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Parenthesized(parenthesized) => parenthesized.to_tokens(tokens),
            Self::ImplTraitOneBound(impl_trait_one_bound) => impl_trait_one_bound.to_tokens(tokens),
            Self::TraitObjectOneBound => {
                todo!("NoBounds::TraitObjectOneBound.to_tokens() not yet implemented")
            }
            Self::Path(path) => path.to_tokens(tokens),
            Self::Tuple => todo!("NoBounds::Tuple.to_tokens() not yet implemented"),
            Self::Never => todo!("NoBounds::Never.to_tokens() not yet implemented"),
            Self::RawPointer => todo!("NoBounds::RawPointer.to_tokens() not yet implemented"),
            Self::Reference => todo!("NoBounds::Reference.to_tokens() not yet implemented"),
            Self::Array => todo!("NoBounds::Array.to_tokens() not yet implemented"),
            Self::Slice => todo!("NoBounds::Slice.to_tokens() not yet implemented"),
            Self::Inferred => todo!("NoBounds::Inferred.to_tokens() not yet implemented"),
            Self::QualifiedPathIn => {
                todo!("NoBounds::QualifiedPathIn.to_tokens() not yet implemented")
            }
            Self::BareFunction => todo!("NoBounds::BareFunction.to_tokens() not yet implemented"),
            Self::MacroInvocation => {
                todo!("NoBounds::MacroInvocation.to_tokens() not yet implemented")
            }
        }
    }
}

impl<'a> From<NoBounds<'a>> for Type<'a> {
    fn from(value: NoBounds<'a>) -> Self {
        Self::NoBounds(value)
    }
}

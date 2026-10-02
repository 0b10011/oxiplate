use super::Type;
use super::impl_trait_one_bound::ImplTraitTypeOneBound;
use super::never::Never;
use super::parenthesized::ParenthesizedType;
use super::path::TypePath;
use super::raw_pointer::RawPointerType;
use super::trait_object_one_bound::TraitObjectTypeOneBound;
use super::tuple::TupleType;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/types.html#railroad-TypeNoBounds>
#[derive(Debug)]
pub(crate) enum TypeNoBounds<'a> {
    /// See: <https://doc.rust-lang.org/reference/types.html#railroad-ParenthesizedType>
    Parenthesized(ParenthesizedType<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/impl-trait.html#railroad-ImplTraitTypeOneBound>
    ImplTraitOneBound(ImplTraitTypeOneBound<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/trait-object.html#railroad-TraitObjectTypeOneBound>
    TraitObjectOneBound(TraitObjectTypeOneBound<'a>),

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypePath>
    Path(TypePath<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/tuple.html#railroad-TupleType>
    Tuple(TupleType<'a>),

    /// `!`
    /// See: <https://doc.rust-lang.org/reference/types/never.html#railroad-NeverType>
    Never(Never<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/pointer.html#railroad-RawPointerType>
    RawPointer(RawPointerType<'a>),

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

impl<'a> TypeNoBounds<'a> {
    /// See: <https://doc.rust-lang.org/reference/types.html#railroad-TypeNoBounds>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((
            into(ParenthesizedType::parse),
            into(ImplTraitTypeOneBound::parse),
            into(TraitObjectTypeOneBound::parse),
            into(TypePath::parse),
            into(TupleType::parse),
            into(Never::parse),
            into(RawPointerType::parse),
        ))
        .parse(tokens)
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::Parenthesized(parenthesized) => parenthesized.source().clone(),
            Self::ImplTraitOneBound(impl_trait_one_bound) => impl_trait_one_bound.source(),
            Self::TraitObjectOneBound(trait_object_type_one_bound) => {
                trait_object_type_one_bound.source()
            }
            Self::Path(type_path) => type_path.source().clone(),
            Self::Tuple(tuple_type) => tuple_type.source().clone(),
            Self::Never(never) => never.source().clone(),
            Self::RawPointer(raw_pointer) => raw_pointer.source(),
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

impl ToTokens for TypeNoBounds<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Parenthesized(parenthesized) => parenthesized.to_tokens(tokens),
            Self::ImplTraitOneBound(impl_trait_one_bound) => impl_trait_one_bound.to_tokens(tokens),
            Self::TraitObjectOneBound(trait_object_type_one_bound) => {
                trait_object_type_one_bound.to_tokens(tokens);
            }
            Self::Path(path) => path.to_tokens(tokens),
            Self::Tuple(tuple_type) => tuple_type.to_tokens(tokens),
            Self::Never(never) => never.to_tokens(tokens),
            Self::RawPointer(raw_pointer) => raw_pointer.to_tokens(tokens),
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

impl<'a> From<TypeNoBounds<'a>> for Type<'a> {
    fn from(value: TypeNoBounds<'a>) -> Self {
        Self::NoBounds(value)
    }
}

use super::Type;
use super::array::ArrayType;
use super::impl_trait::impl_trait_one_bound::ImplTraitTypeOneBound;
use super::inferred::InferredType;
use super::never::Never;
use super::parenthesized::ParenthesizedType;
use super::pointer::raw_pointer::RawPointerType;
use super::pointer::reference::ReferenceType;
use super::slice::SliceType;
use super::trait_object::trait_object_one_bound::TraitObjectTypeOneBound;
use super::tuple::TupleType;
use crate::template::parser::expression::paths::path::TypePath;
use crate::template::parser::expression::paths::qualified_path_in::QualifiedPathInType;
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
    Reference(ReferenceType<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/array.html#railroad-ArrayType>
    Array(ArrayType<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/slice.html#railroad-SliceType>
    Slice(SliceType<'a>),

    /// See: <https://doc.rust-lang.org/reference/types/inferred.html#railroad-InferredType>
    Inferred(InferredType<'a>),

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-QualifiedPathInType>
    QualifiedPathIn(QualifiedPathInType<'a>),

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
            into(ReferenceType::parse),
            into(ArrayType::parse),
            into(SliceType::parse),
            into(InferredType::parse),
            into(QualifiedPathInType::parse),
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
            Self::Reference(reference) => reference.source(),
            Self::Array(array) => array.source().to_owned(),
            Self::Slice(slice) => slice.source().to_owned(),
            Self::Inferred(inferred) => inferred.source().to_owned(),
            Self::QualifiedPathIn(qualified_path) => qualified_path.source(),
            Self::BareFunction => todo!("NoBounds::BareFunction.source() not yet implemented"),
            Self::MacroInvocation => {
                todo!("NoBounds::MacroInvocation.source() not yet implemented")
            }
        }
    }
}

impl<'a> ToTokensWithState<'a> for TypeNoBounds<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::Parenthesized(parenthesized) => parenthesized.to_tokens_with_state(state),
            Self::ImplTraitOneBound(impl_trait_one_bound) => {
                impl_trait_one_bound.to_tokens_with_state(state)
            }
            Self::TraitObjectOneBound(trait_object_type_one_bound) => {
                trait_object_type_one_bound.to_tokens_with_state(state)
            }
            Self::Path(path) => path.to_tokens_with_state(state),
            Self::Tuple(tuple_type) => tuple_type.to_tokens_with_state(state),
            Self::Never(never) => never.to_tokens_with_state(state),
            Self::RawPointer(raw_pointer) => raw_pointer.to_tokens_with_state(state),
            Self::Reference(reference) => reference.to_tokens_with_state(state),
            Self::Array(array) => array.to_tokens_with_state(state),
            Self::Slice(slice) => slice.to_tokens_with_state(state),
            Self::Inferred(inferred) => inferred.to_tokens_with_state(state),
            Self::QualifiedPathIn(qualified_path) => qualified_path.to_tokens_with_state(state),
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

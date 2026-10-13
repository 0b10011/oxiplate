mod args;
mod binding;
mod bounds;
mod lifetime;
mod params;
mod r#type;
mod types;

use self::args::GenericArgList;
use self::types::TypeList;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgs>
#[derive(Debug)]
#[allow(private_interfaces)]
pub(super) enum GenericArgs<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgList>
    List(GenericArgList<'a>),

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypeList>
    Types(TypeList<'a>),
}

impl<'a> GenericArgs<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgs>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((into(GenericArgList::parse), into(TypeList::parse))).parse(tokens)
    }

    pub(super) fn source(&self) -> &Source<'a> {
        match self {
            Self::List(generic_args) => generic_args.source(),
            Self::Types(type_list) => type_list.source(),
        }
    }
}

impl ToTokens for GenericArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::List(generic_args) => generic_args.to_tokens(tokens),
            Self::Types(type_list) => type_list.to_tokens(tokens),
        }
    }
}

pub(in crate::template::parser) mod args;
pub(in crate::template::parser) mod binding;
pub(in crate::template::parser) mod generic_args_bounds;
pub(in crate::template::parser) mod generic_args_const;
pub(in crate::template::parser) mod path;
pub(in crate::template::parser) mod path_segment;
pub(in crate::template::parser) mod qualified_path;
pub(in crate::template::parser) mod qualified_path_in;
pub(in crate::template::parser) mod simple_path_segment;
pub(in crate::template::parser) mod types;

use self::args::GenericArgList;
use self::types::TypeList;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgs>
#[derive(Debug)]
#[allow(private_interfaces)]
pub(in crate::template::parser) enum GenericArgs<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgList>
    List(GenericArgList<'a>),

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypeList>
    Types(TypeList<'a>),
}

impl<'a> GenericArgs<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgs>
    pub(in crate::template::parser) fn parser() -> GenericArgsParser<'a> {
        GenericArgsParser {
            phantom_data: PhantomData,
        }
    }

    pub(super) fn source(&self) -> Source<'a> {
        match self {
            Self::List(generic_args) => generic_args.source().to_owned(),
            Self::Types(type_list) => type_list.source(),
        }
    }
}

impl<'a> ToTokensWithState<'a> for GenericArgs<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::List(generic_args) => generic_args.to_tokens_with_state(state),
            Self::Types(type_list) => type_list.to_tokens_with_state(state),
        }
    }
}

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgs>
pub(in crate::template::parser) struct GenericArgsParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for GenericArgsParser<'a> {
    type TokenKind = TokenKind;
    type Output = GenericArgs<'a>;

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgs>
    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        alt((into(GenericArgList::parser()), into(TypeList::parser()))).parse(tokens)
    }
}

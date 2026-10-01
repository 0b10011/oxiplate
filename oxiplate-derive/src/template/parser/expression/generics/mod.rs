mod args;
mod binding;
mod lifetime;
mod params;
mod r#type;

use self::args::GenericArgs;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgs>
#[derive(Debug)]
#[allow(private_interfaces)]
pub(super) enum Generics<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgList>
    GenericArgs(GenericArgs<'a>),

    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-TypeList>
    #[expect(dead_code, reason = "Not yet implemented")]
    Types,
}

impl<'a> Generics<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgs>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        into(GenericArgs::parse).parse(tokens)
    }

    pub(super) fn source(&self) -> &Source<'a> {
        match self {
            Self::GenericArgs(generic_args) => generic_args.source(),
            Self::Types => todo!("Generics::Types.source() not yet implemented"),
        }
    }
}

impl ToTokens for Generics<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::GenericArgs(generic_args) => generic_args.to_tokens(tokens),
            Self::Types => todo!("Generics::Types.to_tokens() not yet implemented"),
        }
    }
}

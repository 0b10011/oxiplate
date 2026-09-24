mod args;
mod lifetime;

use proc_macro2::TokenStream;
use quote::ToTokens;

use self::args::GenericArgs;
use super::Res;
use crate::Source;
use crate::parser::{Parser as _, into};
use crate::template::tokenizer::TokenSlice;

/// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgs>
#[derive(Debug)]
#[allow(private_interfaces)]
pub(super) enum Generics<'a> {
    GenericArgs(GenericArgs<'a>),
}

impl<'a> Generics<'a> {
    /// See: <https://doc.rust-lang.org/reference/paths.html#railroad-GenericArgs>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        into(GenericArgs::parse).parse(tokens)
    }

    pub(super) fn source(&self) -> &Source<'a> {
        match self {
            Self::GenericArgs(generic_args) => generic_args.source(),
        }
    }
}

impl ToTokens for Generics<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::GenericArgs(generic_args) => generic_args.to_tokens(tokens),
        }
    }
}

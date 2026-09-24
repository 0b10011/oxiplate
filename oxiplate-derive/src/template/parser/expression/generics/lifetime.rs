use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::parser::{Parser as _, take};
use crate::template::parser::Res;
use crate::template::parser::expression::generics::args::GenericArg;
use crate::template::tokenizer::TokenKind;
use crate::{Source, TokenSlice};

/// See <https://doc.rust-lang.org/reference/tokens.html#lifetimes-and-loop-labels>
#[derive(Debug)]
pub(super) struct Lifetime<'a>(Source<'a>);

impl<'a> Lifetime<'a> {
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-Lifetime>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, lifetime) = take(TokenKind::Lifetime).parse(tokens)?;

        Ok((tokens, Self(lifetime.source().clone())))
    }

    pub(super) fn source(&self) -> &Source<'a> {
        &self.0
    }
}

impl ToTokens for Lifetime<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lifetime = syn::Lifetime::new(self.0.as_str(), self.0.span_token());
        lifetime.to_tokens(tokens);
    }
}

impl<'a> From<Lifetime<'a>> for GenericArg<'a> {
    fn from(value: Lifetime<'a>) -> Self {
        Self::Lifetime(value)
    }
}

use proc_macro2::{Ident, Punct};

use super::args::GenericArg;
use crate::template::parser::expression::Identifier;
use crate::template::parser::prelude::*;

/// See <https://doc.rust-lang.org/reference/tokens.html#lifetimes-and-loop-labels>
#[derive(Debug)]
pub(super) struct Lifetime<'a> {
    apostrophe: Source<'a>,
    identifier: Identifier<'a>,
}

impl<'a> Lifetime<'a> {
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-Lifetime>
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (apostrophe, identifier)) =
            (take(TokenKind::Apostrophe), Identifier::parse).parse(tokens)?;

        Ok((
            tokens,
            Self {
                apostrophe: apostrophe.source().clone(),
                identifier,
            },
        ))
    }

    pub(super) fn source(&self) -> Source<'a> {
        self.apostrophe
            .clone()
            .merge(self.identifier.source(), "Identifier should follow `'`")
    }
}

impl ToTokens for Lifetime<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut apostrophe = Punct::new('\'', proc_macro2::Spacing::Joint);
        apostrophe.set_span(self.apostrophe.span_token());
        apostrophe.to_tokens(tokens);

        let identifier_source = self.identifier.source();
        let identifier = Ident::new(identifier_source.as_str(), identifier_source.span_token());
        identifier.to_tokens(tokens);
    }
}

impl<'a> From<Lifetime<'a>> for GenericArg<'a> {
    fn from(value: Lifetime<'a>) -> Self {
        Self::Lifetime(value)
    }
}

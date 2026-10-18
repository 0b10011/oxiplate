use proc_macro2::{Ident, Punct};

use crate::template::parser::expression::Identifier;
use crate::template::parser::expression::paths::args::GenericArg;
use crate::template::parser::prelude::*;

/// See <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-Lifetime>
#[derive(Debug)]
pub(in crate::template::parser) struct Lifetime<'a> {
    apostrophe: Source<'a>,
    identifier: Identifier<'a>,
}

impl<'a> Lifetime<'a> {
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-Lifetime>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (apostrophe, identifier)) = (
            take(TokenKind::Apostrophe),
            cut("Expected identifier", Identifier::parse),
        )
            .parse(tokens)?;

        Ok((
            tokens,
            Self {
                apostrophe: apostrophe.source().clone(),
                identifier,
            },
        ))
    }

    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
        self.apostrophe
            .clone()
            .merge(self.identifier.source(), "Identifier should follow `'`")
    }
}

impl<'a> ToTokensWithState<'a> for Lifetime<'a> {
    fn to_tokens_with_state(&self, _state: &State<'a>) -> BuiltTokens {
        let mut apostrophe = Punct::new('\'', proc_macro2::Spacing::Joint);
        apostrophe.set_span(self.apostrophe.span_token());

        let identifier_source = self.identifier.source();
        let identifier = Ident::new(identifier_source.as_str(), identifier_source.span_token());

        (quote! { #apostrophe #identifier }, EstimatedLength::new(0))
    }
}

impl<'a> From<Lifetime<'a>> for GenericArg<'a> {
    fn from(value: Lifetime<'a>) -> Self {
        Self::Lifetime(value)
    }
}

use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote_spanned;

use super::Pattern;
use crate::parser::{Parser as _, cut, take};
use crate::template::parser::Res;
use crate::template::parser::expression::Identifier;
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{Source, State};

#[derive(Debug)]
pub(crate) struct Bind<'a> {
    ident: Identifier<'a>,
    at: Source<'a>,
    pattern: Box<Pattern<'a>>,
    source: Source<'a>,
}

impl<'a> Bind<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (ident, at, pattern)) = (
            Identifier::parse,
            take(TokenKind::At),
            cut("Expected patter after `@`", Pattern::parse),
        )
            .parse(tokens)?;

        let source = ident
            .source()
            .clone()
            .merge(at.source(), "Expected `@` after identifier")
            .merge(pattern.source(), "Expected pattern after `@`");

        Ok((
            tokens,
            Self {
                ident,
                at: at.source().clone(),
                pattern: Box::new(pattern),
                source,
            },
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        HashSet::from([self.ident.as_str()])
    }

    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let ident = &self.ident;
        let span = self.at.span_token();
        let pattern = self.pattern.to_tokens(state);
        quote_spanned! {span=> #ident @ #pattern }
    }
}

impl<'a> From<Bind<'a>> for Pattern<'a> {
    fn from(value: Bind<'a>) -> Self {
        Self::Bind(value)
    }
}

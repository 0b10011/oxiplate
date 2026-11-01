use std::collections::HashSet;

use super::Pattern;
use crate::template::parser::expression::Identifier;
use crate::template::parser::prelude::*;

#[derive(Debug)]
pub(crate) struct Bind<'a> {
    ident: Identifier<'a>,
    at: Source<'a>,
    pattern: Box<Pattern<'a>>,
    source: Source<'a>,
}

impl<'a> Bind<'a> {
	pub(in crate::template::parser) fn parser() -> BindParser<'a> {
	    BindParser {
	        phantom_data: PhantomData,
	    }
	}

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        HashSet::from([self.ident.as_str()])
    }
}

impl<'a> ToTokensWithState<'a> for Bind<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let ident = &self.ident;
        let span = self.at.span_token();
        let (pattern, estimated_length) = self.pattern.to_tokens_with_state(state);

        (quote_spanned! {span=> #ident @ #pattern }, estimated_length)
    }
}

impl<'a> From<Bind<'a>> for Pattern<'a> {
    fn from(value: Bind<'a>) -> Self {
        Self::Bind(value)
    }
}

pub(in crate::template::parser) struct BindParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for BindParser<'a> {
    type TokenKind = TokenKind;
    type Output = Bind<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (ident, at, pattern)) = (
            Identifier::parser(),
            take(TokenKind::At),
            cut("Expected patter after `@`", Pattern::parser()),
        )
            .parse(tokens)?;

        let source = ident
            .source()
            .clone()
            .merge(at.source(), "Expected `@` after identifier")
            .merge(&pattern.source(), "Expected pattern after `@`");

        Ok((
            tokens,
            Bind {
                ident,
                at: at.source().clone(),
                pattern: Box::new(pattern),
                source,
            },
        ))
    }
}

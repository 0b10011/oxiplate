use std::marker::PhantomData;

use super::prelude::*;
use super::value::StringValue;

pub(in crate::config::parser) fn string<'a>() -> StringParser<'a> {
    StringParser {
        phantom_data: PhantomData,
    }
}
pub(in crate::config::parser) struct StringParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for StringParser<'a> {
    type TokenKind = TokenKind;
    type Output = StringValue<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, token) = tokens.take()?;

        let value = match token.kind() {
            TokenKind::String(value) => StringValue::new(value, token.source()),
            _ => {
                return Err(Error::recoverable(
                    format!(r"Expected a string, found: {token:#?}"),
                    token.source().clone(),
                ));
            }
        };

        Ok((tokens, value))
    }
}

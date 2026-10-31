use std::marker::PhantomData;

use super::prelude::*;

pub(in crate::config::parser) fn newline_or_eof<'a>() -> NewlineOrEofParser<'a> {
    NewlineOrEofParser {
        phantom_data: PhantomData,
    }
}
pub(in crate::config::parser) struct NewlineOrEofParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for NewlineOrEofParser<'a> {
    type TokenKind = TokenKind;
    type Output = &'a Source<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        if tokens.is_empty() {
            let eof = tokens.eof().source();
            return Ok((tokens, eof));
        }

        let (tokens, token) = tokens.take()?;

        match token.kind() {
            TokenKind::Newline => (),
            _ => {
                return Err(Error::recoverable(
                    format!(r"Expected a newline (`\n` or `\r\n`), found: {token:#?}"),
                    token.source().clone(),
                ));
            }
        }

        Ok((tokens, token.source()))
    }
}

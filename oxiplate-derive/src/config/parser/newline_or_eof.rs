use std::marker::PhantomData;

use super::prelude::*;

basic_parser! {
    pub(in crate::config::parser) fn newline_or_eof(tokens: <TokenKind>) -> &'a Source<'a> {
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

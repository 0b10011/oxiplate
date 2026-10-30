use super::prelude::*;
use super::value::StringValue;

basic_parser! {
    pub(in crate::config::parser) fn string(tokens: <TokenKind>) -> StringValue<'a> {
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

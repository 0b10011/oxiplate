use quote::quote_spanned;

use super::{Item, Res};
use crate::syntax::parser::{Parser as _, alt, take};
use crate::tokenizer::parser::{TokenKind, TokenSlice};
use crate::{BuiltTokens, Source};

#[derive(Debug)]
pub(crate) struct Static<'a>(pub &'a str, pub Source<'a>);

impl Static<'_> {
    pub fn to_token(&self) -> BuiltTokens {
        let text = &self.0;
        let span = self.1.span_token();
        (quote_spanned! { span => #text }, text.len())
    }
}

/// Type of static text.
#[derive(Debug)]
pub(crate) enum StaticType {
    /// One or more whitespace characters.
    /// Tracked separately to allow for it
    /// to appear in the top level when extending templates.
    Whitespace,

    /// Plain text that may contain whitespace.
    Text,
}

pub(crate) fn parse_static(tokens: TokenSlice) -> Res<Vec<Item>> {
    let (tokens, item) = alt((
        take(TokenKind::StaticText),
        take(TokenKind::StaticWhitespace),
    ))
    .parse(tokens)?;

    Ok((
        tokens,
        vec![Item::Static(
            Static(item.source().as_str(), item.source().clone()),
            if let TokenKind::StaticWhitespace = *item.kind() {
                StaticType::Whitespace
            } else {
                StaticType::Text
            },
        )],
    ))
}

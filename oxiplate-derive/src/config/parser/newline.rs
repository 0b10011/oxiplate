use super::item::Item;
use super::prelude::*;

basic_parser! {
    pub(in crate::config::parser) fn newline(tokens: <TokenKind>) -> Item<'a> {
        let (tokens, newline) = take(TokenKind::Newline).parse(tokens)?;

        Ok((tokens, Item::Newline(newline.source().clone())))
    }
}

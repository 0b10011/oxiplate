use super::item::Item;
use super::prelude::*;

pub(in crate::config::parser) fn newline<'a>(tokens: TokenSlice<'a>) -> Res<'a, Item<'a>> {
    let (tokens, newline) = take(TokenKind::Newline).parse(tokens)?;

    Ok((tokens, Item::Newline(newline.source().clone())))
}

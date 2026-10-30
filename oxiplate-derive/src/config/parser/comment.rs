use super::item::Item;
use super::newline_or_eof::newline_or_eof;
use crate::config::parser::prelude::*;

pub(in crate::config::parser) fn comment<'a>(tokens: TokenSlice<'a>) -> Res<'a, Item<'a>> {
    let (tokens, (comment, newline)) = (
        take(TokenKind::Comment),
        cut(
            "Newline or end of file expected after comment",
            newline_or_eof(),
        ),
    )
        .parse(tokens)?;

    Ok((
        tokens,
        Item::Comment(
            comment
                .source()
                .clone()
                .merge(newline, "Newline expected after comment"),
        ),
    ))
}

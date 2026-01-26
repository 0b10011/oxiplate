use super::item::tag_end;
use super::{Item, Res};
use crate::syntax::parser::{Parser as _, cut, many0, take};
use crate::tokenizer::{TagKind, TokenKind};
use crate::{Source, TokenSlice};

pub(super) fn comment<'a>(
    open_tag_source: Source<'a>,
) -> impl Fn(TokenSlice<'a>) -> Res<'a, (Item<'a>, Option<Item<'a>>)> {
    move |tokens| {
        let (tokens, (comment_text, (trailing_whitespace, close_tag))) = (
            many0(take(TokenKind::Comment)),
            cut("Expected `#}`, `-#}`, or `_#}`", tag_end(TagKind::Comment)),
        )
            .parse(tokens)?;

        let mut comment = open_tag_source.clone();

        for text in comment_text {
            comment = comment.merge(
                text.source(),
                "Comment text should follow open tag or previous comment text",
            );
        }

        comment = comment.merge(&close_tag, "Close tag expected after comment text");

        Ok((tokens, (Item::Comment(comment), trailing_whitespace)))
    }
}

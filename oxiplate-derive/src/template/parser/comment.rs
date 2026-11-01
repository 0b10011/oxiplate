use super::Item;
use super::item::tag_end;
use crate::template::parser::prelude::*;
use crate::template::tokenizer::TagKind;

pub(in crate::template::parser) fn comment(open_tag_source: Source) -> CommentParser {
    CommentParser { open_tag_source }
}
pub(in crate::template::parser) struct CommentParser<'a> {
    open_tag_source: Source<'a>,
}

impl<'a> Parser<'a> for CommentParser<'a> {
    type TokenKind = TokenKind;
    type Output = (Item<'a>, Option<Item<'a>>);

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (comment_text, (trailing_whitespace, close_tag))) =
            (many0(take(TokenKind::Comment)), tag_end(TagKind::Comment)).parse(tokens)?;

        let mut comment = self.open_tag_source.clone();

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

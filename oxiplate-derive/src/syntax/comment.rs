use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1};
use nom::error::context;
use nom::multi::many_till;

use super::item::tag_end;
use super::{Item, Res};
use crate::Source;

pub(super) fn comment<'a>(
    open_tag_source: Source<'a>,
) -> impl Fn(Source<'a>) -> Res<Source<'a>, (Item<'a>, Option<Item<'a>>)> {
    move |input| {
        let (input, (comment_parts, (trailing_whitespace, close_tag))) = context(
            "Expected comment text followed by `#}`",
            many_till(
                alt((
                    take_till1(|char| char == '-' || char == '_' || char == '#'),
                    tag("-"),
                    tag("_"),
                    tag("#"),
                )),
                tag_end("#}"),
            ),
        )
        .parse(input)?;

        let mut comment = open_tag_source.clone();
        for part in comment_parts {
            comment = comment.merge(&part, "Comment part should follow previous part");
        }
        comment = comment.merge(&close_tag, "Close tag expected after comment text");

        Ok((input, (Item::Comment(comment), trailing_whitespace)))
    }
}

use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1};
use nom::multi::many_till;

use super::item::tag_end;
use super::{Item, Res};
use crate::Source;

#[derive(Debug, PartialEq, Eq)]
pub struct Comment<'a>(&'a str);

impl<'a> From<Comment<'a>> for Item<'a> {
    fn from(_comment: Comment) -> Self {
        Item::Comment
    }
}

pub(super) fn comment(input: Source) -> Res<Source, (Item, Option<Item>)> {
    let (input, (_comment, (trailing_whitespace, _close_tag))) = many_till(
        alt((
            take_till1(|char| char == '-' || char == '_' || char == '#'),
            tag("-"),
            tag("_"),
            tag("#"),
        )),
        tag_end("#}"),
    )
    .parse(input)?;

    Ok((input, (Item::Comment, trailing_whitespace)))
}

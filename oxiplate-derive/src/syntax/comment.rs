use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1};
use nom::error::context;
use nom::multi::many_till;

use super::item::tag_end;
use super::{Item, Res};
use crate::Source;

pub(super) fn comment(input: Source) -> Res<Source, (Item, Option<Item>)> {
    let (input, (_comment, (trailing_whitespace, _close_tag))) = context(
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

    Ok((input, (Item::Comment, trailing_whitespace)))
}

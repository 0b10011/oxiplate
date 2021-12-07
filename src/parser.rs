use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::multi::many0;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Template<'a>(Vec<Item<'a>>);

#[derive(Debug, PartialEq)]
pub enum Item<'a> {
    Tag(Tag<'a>),
    Static(Static<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Tag<'a>(&'a str);

impl<'a> From<Tag<'a>> for Item<'a> {
    fn from(tag: Tag<'a>) -> Self {
        Item::Tag(tag)
    }
}

#[derive(Debug, PartialEq)]
pub struct Static<'a>(&'a str);

impl<'a> From<Static<'a>> for Item<'a> {
    fn from(r#static: Static<'a>) -> Self {
        Item::Static(r#static)
    }
}

pub fn parse(input: &str) -> IResult<&str, Template> {
    let (input, items) = many0(alt((parse_tag, parse_static)))(input)?;

    // Return error if there's any input remaining.
    // Successful value is `("", "")`, so no need to capture.
    eof(input)?;

    Ok(("", Template(items)))
}

pub fn parse_tag(input: &str) -> IResult<&str, Item> {
    let (input, text) = tag("foo")(input)?;

    Ok((input, Item::Tag(Tag(text))))
}

pub fn parse_static(input: &str) -> IResult<&str, Item> {
    let (input, text) = tag("bar")(input)?;

    Ok((input, Item::Static(Static(text))))
}

#[test]
fn test_empty() {
    assert_eq!(parse(""), Ok(("", Template(vec![]))));
}

#[test]
fn test_fake() {
    assert_eq!(
        parse("Some text."),
        Ok(("", Template(vec![Item::Static(Static("Some text."))])))
    );
}

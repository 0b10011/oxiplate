use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_while1};
use nom::character::complete::char;
use nom::combinator::{eof, opt, peek};
use nom::multi::{many0, many_till};
use nom::sequence::pair;
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
pub struct Static<'a>(Vec<&'a str>);

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

// https://doc.rust-lang.org/reference/whitespace.html
fn is_whitespace(char: char) -> bool {
    match char {
        '\u{0009}' // (horizontal tab, '\t')
        | '\u{000A}' // (line feed, '\n')
        | '\u{000B}' // (vertical tab)
        | '\u{000C}' // (form feed)
        | '\u{000D}' // (carriage return, '\r')
        | '\u{0020}' // (space, ' ')
        | '\u{0085}' // (next line)
        | '\u{200E}' // (left-to-right mark)
        | '\u{200F}' // (right-to-left mark)
        | '\u{2028}' // (line separator)
        | '\u{2029}' // (paragraph separator)
        => true,
        _ => false,
    }
}

fn whitespace(input: &str) -> IResult<&str, &str> {
    take_while1(is_whitespace)(input)
}

#[derive(Debug)]
pub enum TagOpen {
    Writ,
    Statement,
    Comment,
}

pub fn tag_open(input: &str) -> IResult<&str, TagOpen> {
    let (input, output) = alt((
        tag("{{"), // writ
        tag("{%"), // statement
        tag("{#"), // comment
    ))(input)?;

    match output {
        "{{" => Ok((input, TagOpen::Writ)),
        "{%" => Ok((input, TagOpen::Statement)),
        "{#" => Ok((input, TagOpen::Comment)),
        _ => panic!("This should never happen"),
    }
}

fn writ(input: &str) -> IResult<&str, Item> {
    let (input, output) = tag("test")(input)?;

    Ok((input, Item::Tag(Tag(output))))
}

fn statement(input: &str) -> IResult<&str, Item> {
    let (input, output) = tag("test")(input)?;

    Ok((input, Item::Tag(Tag(output))))
}

fn comment(input: &str) -> IResult<&str, Item> {
    let (input, output) = tag("test")(input)?;

    Ok((input, Item::Tag(Tag(output))))
}

fn parse_tag(input: &str) -> IResult<&str, Item> {
    match tag_open(input)? {
        (input, TagOpen::Writ) => writ(input),
        (input, TagOpen::Statement) => statement(input),
        (input, TagOpen::Comment) => comment(input),
    }
}

fn is_whitespace_or_brace(char: char) -> bool {
    char == '{' || is_whitespace(char)
}

pub fn collapse_whitespace_command(input: &str) -> IResult<&str, char> {
    char('_')(input)
}

pub fn trim_whitespace_command(input: &str) -> IResult<&str, char> {
    char('-')(input)
}

pub fn parse_static(input: &str) -> IResult<&str, Item> {
    let (input, (output, (whitespace, (open_tag, command)))) = many_till(
        take_till1(is_whitespace_or_brace),
        pair(
            whitespace,
            peek(pair(
                tag_open,
                opt(alt((collapse_whitespace_command, trim_whitespace_command))),
            )),
        ),
    )(input)?;

    Ok((input, Item::Static(Static(output))))
}

#[test]
fn test_empty() {
    assert_eq!(parse(""), Ok(("", Template(vec![]))));
}

#[test]
fn test_word() {
    assert_eq!(
        parse("Test"),
        Ok(("", Template(vec![Item::Static(Static(vec!["Test"]))])))
    );
}

#[test]
fn test_phrase() {
    assert_eq!(
        parse("Some text."),
        Ok(("", Template(vec![Item::Static(Static(vec!["Some text."]))])))
    );
}
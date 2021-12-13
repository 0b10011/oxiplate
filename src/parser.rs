use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_while1};
use nom::character::complete::char;
use nom::combinator::{eof, fail, opt, peek, recognize};
use nom::error::VerboseError;
use nom::multi::{many0, many_till};
use nom::sequence::tuple;
use nom::Err as SynErr;

type Res<T, U> = nom::IResult<T, U, VerboseError<T>>;

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

pub fn parse(input: &str) -> Res<&str, Template> {
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

fn whitespace(input: &str) -> Res<&str, &str> {
    take_while1(is_whitespace)(input)
}

#[derive(Debug)]
pub enum TagOpen {
    Writ,
    Statement,
    Comment,
}

pub fn tag_open(input: &str) -> Res<&str, TagOpen> {
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

fn writ(input: &str) -> Res<&str, Item> {
    let (input, output) = tag("test")(input)?;

    Ok((input, Item::Tag(Tag(output))))
}

fn statement(input: &str) -> Res<&str, Item> {
    let (input, output) = tag("test")(input)?;

    Ok((input, Item::Tag(Tag(output))))
}

fn comment(input: &str) -> Res<&str, Item> {
    let (input, output) = tag("test")(input)?;

    Ok((input, Item::Tag(Tag(output))))
}

fn parse_tag(input: &str) -> Res<&str, Item> {
    let tag = match tag_open(input)? {
        (input, TagOpen::Writ) => writ(input),
        (input, TagOpen::Statement) => statement(input),
        (input, TagOpen::Comment) => comment(input),
    };

    match tag {
        Err(SynErr::Error(error)) => Err(SynErr::Failure(error)),
        error => error,
    }
}

fn is_whitespace_or_brace(char: char) -> bool {
    char == '{' || is_whitespace(char)
}

pub fn collapse_whitespace_command(input: &str) -> Res<&str, char> {
    char('_')(input)
}

pub fn trim_whitespace_command(input: &str) -> Res<&str, char> {
    char('-')(input)
}

pub fn parse_static(input: &str) -> Res<&str, Item> {
    let (input, (output, _)) = many_till(
        take_till1(is_whitespace_or_brace),
        peek(alt((
            recognize(tuple((
                opt(whitespace),
                tag_open,
                opt(alt((collapse_whitespace_command, trim_whitespace_command))),
            ))),
            eof
        ))),
    )(input)?;

    // Must be checked for many0() call will fail due to infinite loop
    if output.is_empty() {
        return fail(input);
    }

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

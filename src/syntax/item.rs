use super::{
    comment::comment, statement::statement, template::whitespace, writ::writ, Res, Span, Statement,
    Static, Writ,
};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, opt};
use nom::error::VerboseError;
use nom::sequence::tuple;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, PartialEq, Eq)]
pub enum Item<'a> {
    Comment,
    Writ(Writ<'a>),
    Statement(Statement<'a>),
    Static(Static),
    CompileError(String),
}

impl ToTokens for Item<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Item::Comment => quote! {},
            Item::Writ(writ) => quote! { #writ },
            Item::Statement(statement) => quote! { #statement },
            Item::Static(text) => quote! { #text },
            Item::CompileError(text) => quote! { compile_error!(#text); },
        });
    }
}

#[derive(Debug)]
pub enum TagOpen {
    Writ,
    Statement,
    Comment,
}

pub fn parse_tag(input: Span) -> Res<&str, Vec<Item>> {
    let (input, (leading_whitespace, open)) = tag_start(input)?;

    let parser = match open {
        TagOpen::Writ => writ,
        TagOpen::Statement => statement,
        TagOpen::Comment => comment,
    };
    let (input, (tag, trailing_whitespace)) = cut(parser)(input)?;

    let mut items = vec![];

    if let Some(leading_whitespace) = leading_whitespace {
        items.push(leading_whitespace.into());
    }

    items.push(tag);

    if let Some(trailing_whitespace) = trailing_whitespace {
        items.push(trailing_whitespace.into());
    }

    Ok((input, items))
}

pub fn tag_start(input: Span) -> Res<&str, (Option<Static>, TagOpen)> {
    let (input, (whitespace, open, command)) = tuple((
        // Whitespace is optional, but tracked because it could be altered by tag.
        opt(whitespace),
        // Check if this is actually a tag; if it's not, that's fine, just return early.
        tag_open,
        // Whitespace control characters are optional.
        opt(alt((collapse_whitespace_command, trim_whitespace_command))),
    ))(input)?;

    let whitespace = match command {
        // Collapse to a single space if there's any leading whitespace.
        Some('_') => whitespace.map(|_| Static(" ".to_string())),
        // Remove any leading whitespace.
        Some('-') => None,
        Some(_) => unreachable!("Only - or _ should be matched"),
        // Convert any leading whitespace to `Static()` without adjusting.
        None => whitespace.map(|whitespace| Static(whitespace.to_string())),
    };

    Ok((input, (whitespace, open)))
}

pub fn tag_end(tag_close: &str) -> impl Fn(Span) -> Res<&str, Option<Static>> + '_ {
    move |input| {
        if let Ok((input, _tag)) = tag::<_, _, VerboseError<_>>(tag_close)(input) {
            return Ok((input, None));
        }

        let (input, (command, _, whitespace)) = tuple((
            alt((collapse_whitespace_command, trim_whitespace_command)),
            tag(tag_close),
            opt(whitespace),
        ))(input)?;

        let whitespace = match command {
            '_' => whitespace.map(|_| Static(" ".to_string())),
            '-' => None,
            _ => unreachable!("Only - or _ should be matched"),
        };

        Ok((input, whitespace))
    }
}

pub fn tag_open(input: Span) -> Res<&str, TagOpen> {
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

fn collapse_whitespace_command(input: Span) -> Res<&str, char> {
    char('_')(input)
}

fn trim_whitespace_command(input: Span) -> Res<&str, char> {
    char('-')(input)
}

use super::{
    comment::comment, statement::statement, template::whitespace, writ::writ, Res, Span, Statement,
    Static, Writ,
};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{fail, opt};
use nom::error::VerboseError;
use nom::sequence::tuple;
use nom::Err as NomErr;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, PartialEq, Eq)]
pub enum Item<'a> {
    Comment,
    Writ(Writ<'a>),
    Statement(Statement<'a>),
    Static(Static),
}

impl ToTokens for Item<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Item::Comment => quote! {},
            Item::Writ(writ) => quote! { #writ },
            Item::Statement(statement) => quote! { #statement },
            Item::Static(text) => quote! { #text },
        });
    }
}

#[derive(Debug)]
pub enum TagOpen {
    Writ,
    Statement,
    Comment,
}

pub fn parse_tag(_variables: &'_ [&syn::Ident]) -> impl Fn(Span) -> Res<&str, Vec<Item>> {
    |input| {
        let (input, (leading_whitespace, open)) = tag_start(input)?;

        let tag = match open {
            TagOpen::Writ => writ(input),
            TagOpen::Statement => statement(input),
            TagOpen::Comment => comment(input),
        };

        match tag {
            Err(NomErr::Error(error)) => Err(NomErr::Failure(error)),
            Ok((input, (tag, trailing_whitespace))) => {
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
            Err(error) => Err(error),
        }
    }
}

pub fn tag_start(input: Span) -> Res<&str, (Option<Static>, TagOpen)> {
    if let Ok((input, tag)) = tag_open(input) {
        return Ok((input, (None, tag)));
    }

    let (input, (_, open, command)) = tuple((
        opt(whitespace),
        tag_open,
        alt((collapse_whitespace_command, trim_whitespace_command)),
    ))(input)?;

    let whitespace = match command {
        '_' => Some(Static(" ".to_owned())),
        '-' => None,
        _ => return fail(input),
    };

    Ok((input, (whitespace, open)))
}

pub fn tag_end(tag_close: &str) -> impl Fn(Span) -> Res<&str, Option<Static>> + '_ {
    move |input| {
        if let Ok((input, _tag)) = tag::<_, _, VerboseError<_>>(tag_close)(input) {
            return Ok((input, None));
        }

        let (input, (command, _, _)) = tuple((
            alt((collapse_whitespace_command, trim_whitespace_command)),
            tag(tag_close),
            opt(whitespace),
        ))(input)?;

        let whitespace = match command {
            '_' => Some(Static(" ".to_owned())),
            '-' => None,
            _ => return fail(input),
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

    match *output.fragment() {
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

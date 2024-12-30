use crate::State;

use super::{super::Source, item::parse_tag, r#static::parse_static, Item, Res, Static};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{eof, opt};
use nom::error::VerboseErrorKind;
use nom::multi::many0;
use nom::sequence::tuple;
use proc_macro2::LineColumn;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};

#[derive(Debug)]
pub(crate) struct Template<'a>(pub Vec<Item<'a>>);

impl ToTokens for Template<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.0);
    }
}

pub(crate) fn parse<'a>(state: &'a State<'a>, source: Source<'a>) -> Template<'a> {
    match try_parse(state, source) {
        Ok((_, template)) => template,
        Err(
            nom::Err::Error(nom::error::VerboseError { errors })
            | nom::Err::Failure(nom::error::VerboseError { errors }),
        ) => Template(vec![convert_error(errors)]),
        Err(nom::Err::Incomplete(_)) => {
            unreachable!("This should only happen in nom streams which aren't used by Oxiplate.")
        }
    }
}

fn convert_error(errors: Vec<(Source, VerboseErrorKind)>) -> Item {
    use std::fmt::Write;

    let mut converted_error = String::from("Backtrace:\n");
    let mut last_source = None;

    for (source, kind) in errors {
        match kind {
            VerboseErrorKind::Char(expected_char) => {
                let LineColumn { line, column } = source.span().start();
                writeln!(
                    &mut converted_error,
                    "[line {}, column {}] Expected '{}', found '{}'",
                    line,
                    column,
                    expected_char,
                    source.as_str()
                )
                .unwrap();
            }
            VerboseErrorKind::Context(error) => {
                return Item::CompileError(error.to_string(), source)
            }
            VerboseErrorKind::Nom(nom_error) => {
                let LineColumn { line, column } = source.span().start();
                writeln!(
                    &mut converted_error,
                    r#"[line {}, column {}] {:?} in "{}""#,
                    line,
                    column,
                    nom_error,
                    source.as_str()
                )
                .unwrap();
            }
        }
        last_source = Some(source);
    }

    Item::CompileError(
        converted_error,
        last_source.expect("There should be at least one source listed in an error"),
    )
}

fn try_parse<'a>(state: &'a State<'a>, source: Source<'a>) -> Res<Source<'a>, Template<'a>> {
    let (input, items_vec) = many0(parse_item(state, &true))(source)?;

    // Return error if there's any input remaining.
    // Successful value is `("", "")`, so no need to capture.
    let (input, _) = eof(input)?;

    let mut items = Vec::new();
    for mut item_vec in items_vec {
        items.append(&mut item_vec);
    }

    let mut has_content = false;
    let mut is_extending = false;
    for item in &items {
        match item {
            Item::Statement(statement) => {
                match statement.kind {
                    crate::syntax::statement::StatementKind::Extends(_) => {
                        if has_content || is_extending {
                            todo!("Can't extend if already adding content");
                        }

                        is_extending = true;
                    }
                    crate::syntax::statement::StatementKind::Block(_) => {
                        // While blocks are allowed when extending,
                        // the extends tag should cause an error if it appears _after_ a block.
                        has_content = true;
                    }
                    _ => {
                        if is_extending {
                            todo!("Can't add content if extending");
                        }

                        has_content = true;
                    }
                }
            }
            #[allow(clippy::match_same_arms)]
            Item::Writ(_) => (),
            Item::Static(_) => {
                if is_extending {
                    todo!("Can't add static content or writs when extending");
                }

                // Whitespace-only `Static` should be wrapped in `Item::Whitespace()` instead,
                // so no check needs to be done for whitespace.
                has_content = true;
            }
            // These are fine anywhere
            Item::CompileError(_, _) | Item::Comment | Item::Whitespace(_) => (),
        }
    }

    Ok((input, Template(items)))
}

pub(crate) fn parse_item<'a>(
    state: &'a State,
    should_output_blocks: &'a bool,
) -> impl Fn(Source) -> Res<Source, Vec<Item>> + 'a {
    |input| {
        alt((
            parse_tag(state, should_output_blocks),
            parse_static,
            adjusted_whitespace,
        ))(input)
    }
}

pub(crate) fn adjusted_whitespace(input: Source) -> Res<Source, Vec<Item>> {
    let (input, (leading_whitespace, tag, trailing_whitespace)) = tuple((
        opt(whitespace),
        alt((tag("{_}"), tag("{-}"))),
        opt(whitespace),
    ))(input)?;

    let whitespace = match tag.as_str() {
        "{_}" => {
            if let Some(leading_whitespace) = leading_whitespace {
                vec![Item::Whitespace(Static(" ", leading_whitespace))]
            } else if let Some(trailing_whitespace) = trailing_whitespace {
                vec![Item::Whitespace(Static(" ", trailing_whitespace))]
            } else {
                vec![]
            }
        }
        "{-}" => vec![],
        _ => unreachable!("Only whitespace control tags should be matched"),
    };

    Ok((input, whitespace))
}

// https://doc.rust-lang.org/reference/whitespace.html
pub fn is_whitespace(char: char) -> bool {
    matches!(
        char,
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
    )
}

pub(crate) fn whitespace(input: Source) -> Res<Source, Source> {
    take_while1(is_whitespace)(input)
}

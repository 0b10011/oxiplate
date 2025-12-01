use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{eof, opt};
use nom::multi::many0;
use nom::{Input, Parser as _};
use nom_language::error::{VerboseError, VerboseErrorKind};
use proc_macro2::{LineColumn, TokenStream};
use quote::{TokenStreamExt, quote};

use super::super::Source;
use super::item::{ItemToken, parse_tag};
use super::r#static::parse_static;
use super::{Item, Res, Static};
use crate::State;

/// Collection of items in the template and estimated output length.
#[derive(Debug)]
pub(crate) struct Template<'a>(pub(crate) Vec<Item<'a>>);

impl Template<'_> {
    #[inline]
    fn write_tokens(str_tokens: &mut Vec<TokenStream>, tokens: &mut TokenStream) {
        if str_tokens.is_empty() {
            return;
        }

        let concat_tokens = quote! { concat!(#(#str_tokens),*) };
        str_tokens.clear();

        tokens.append_all(quote! { f.write_str(#concat_tokens)?; });
    }

    pub fn to_tokens(&self, state: &State<'_>) -> (TokenStream, usize) {
        let mut tokens = TokenStream::new();
        let mut estimated_length = 0;

        let mut str_tokens = vec![];
        let mut state = state.clone();
        for item in &self.0 {
            match item.to_token(&mut state) {
                ItemToken::Comment => (),
                ItemToken::StaticText(token_stream, item_length) => {
                    estimated_length += item_length;
                    str_tokens.push(token_stream);
                }
                ItemToken::DynamicText(token_stream, item_length) => {
                    estimated_length += item_length;
                    // Write out static text
                    if !str_tokens.is_empty() {
                        Self::write_tokens(&mut str_tokens, &mut tokens);
                    }

                    tokens.append_all(quote! {
                        #token_stream;
                    });
                }
                ItemToken::Statement(token_stream, item_length) => {
                    estimated_length += item_length;
                    Self::write_tokens(&mut str_tokens, &mut tokens);
                    tokens.append_all(token_stream);
                }
            }
        }

        if !str_tokens.is_empty() {
            Self::write_tokens(&mut str_tokens, &mut tokens);
        }

        (tokens, estimated_length)
    }
}

pub(crate) fn parse(state: &State, source: Source) -> (TokenStream, usize) {
    match try_parse(state, source) {
        Ok((_, template)) => template,
        Err(
            nom::Err::Error(VerboseError { errors }) | nom::Err::Failure(VerboseError { errors }),
        ) => {
            let template = Template(vec![convert_error(errors)]);
            template.to_tokens(state)
        }
        // coverage:ignore-start
        Err(nom::Err::Incomplete(_)) => {
            unreachable!("This should only happen in nom streams which aren't used by Oxiplate.")
        }
        // coverage:ignore-stop
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
                let source = <Source as Input>::take_from(&source, 0);
                return Item::CompileError(error.to_string(), source);
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

fn try_parse<'a>(state: &State, source: Source<'a>) -> Res<Source<'a>, (TokenStream, usize)> {
    let (input, items_vec) = many0(parse_item).parse(source)?;

    // Return error if there's any input remaining.
    // Successful value is `("", "")`, so no need to capture.
    let (input, _) = eof(input)?;

    let mut items = Vec::new();
    for mut item_vec in items_vec {
        items.append(&mut item_vec);
    }

    Ok((input, Template(items).to_tokens(state)))
}

pub(crate) fn parse_item(input: Source) -> Res<Source, Vec<Item>> {
    alt((parse_tag, parse_static, adjusted_whitespace)).parse(input)
}

pub(crate) fn adjusted_whitespace(input: Source) -> Res<Source, Vec<Item>> {
    let (input, (leading_whitespace, tag, trailing_whitespace)) = (
        opt(whitespace),
        alt((tag("{_}"), tag("{-}"))),
        opt(whitespace),
    )
        .parse(input)?;

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
        // coverage:ignore
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
    take_while1(is_whitespace).parse(input)
}

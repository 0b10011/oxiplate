use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{eof, opt};
use nom::multi::many0;
use nom::{Input, Parser as _};
use nom_language::error::{VerboseError, VerboseErrorKind};
use proc_macro::Diagnostic;
use proc_macro2::{LineColumn, TokenStream};
use quote::{TokenStreamExt, quote};

use super::super::Source;
use super::item::{ItemToken, parse_tag};
use super::r#static::parse_static;
use super::{Item, Res, Static};
use crate::State;
use crate::syntax::item::{WhitespacePreference, parse_trailing_whitespace};

/// Collection of items in the template and estimated output length.
#[derive(Debug)]
pub(crate) struct Template<'a>(pub(crate) Vec<Item<'a>>);

impl Template<'_> {
    #[cfg(coverage_nightly)]
    pub fn source(&self) -> Option<Source<'_>> {
        let mut source: Option<Source<'_>> = None;
        for item in &self.0 {
            source = Some(source.map_or(item.source().clone(), |source| {
                source.merge(item.source(), "Item source should follow previous item")
            }));
        }
        source
    }

    #[inline]
    fn write_tokens(str_tokens: &mut Vec<TokenStream>, tokens: &mut TokenStream) {
        if str_tokens.is_empty() {
            return;
        }

        let concat_tokens = quote! { concat!(#(#str_tokens),*) };
        str_tokens.clear();

        tokens.append_all(quote! { oxiplate_formatter.write_str(#concat_tokens)?; });
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
                let source = <Source as Input>::take_from(&source, 0);
                return Item::CompileError {
                    message: error.to_string(),
                    error_source: source.clone(),
                    consumed_source: source,
                };
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

    let consumed_source =
        last_source.expect("There should be at least one source listed in an error");
    Item::CompileError {
        message: converted_error,
        error_source: consumed_source.clone(),
        consumed_source,
    }
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

    let template = Template(items);

    // Ensure all tested items build source properly.
    #[cfg(coverage_nightly)]
    let _ = template.source();

    Ok((input, template.to_tokens(state)))
}

pub(crate) fn parse_item(input: Source) -> Res<Source, Vec<Item>> {
    alt((parse_tag, parse_static, adjusted_whitespace)).parse(input)
}

pub(crate) fn adjusted_whitespace(input: Source) -> Res<Source, Vec<Item>> {
    let (input, (leading_whitespace, tag)) = (
        opt(whitespace),
        alt((
            tag("{_}"),
            tag("{-}"),
            #[cfg(feature = "unreachable")]
            tag("{$}"),
        )),
    )
        .parse(input)?;

    let whitespace_preference = match tag.as_str() {
        "{-}" => WhitespacePreference::Remove,
        "{_}" => WhitespacePreference::Replace,
        _ => {
            Diagnostic::spanned(
                tag.span().unwrap(),
                proc_macro::Level::Error,
                "Internal Oxiplate error: Unhandled whitespace adjustment tag",
            )
            .help("Please open an issue: https://github.com/0b10011/oxiplate/issues/new?title=Unhandled+whitespace+adjustment+tag")
            .help("Include template that caused the issue.")
            .emit();
            unreachable!("Internal Oxiplate error. See previous error for more information.");
        }
    };

    let (input, trailing_whitespace) = parse_trailing_whitespace(
        tag.clone(),
        whitespace_preference,
        leading_whitespace.is_some(),
    )
    .parse(input)?;

    let has_whitespace =
        leading_whitespace.is_some() || matches!(&trailing_whitespace, Some(Item::Whitespace(_)));
    let tag_str = tag.as_str();

    let source = if let Some(leading_whitespace) = &leading_whitespace {
        leading_whitespace
            .clone()
            .merge(&tag, "Tag expected after leading whitespace")
    } else {
        tag.clone()
    }
    .merge_some(
        trailing_whitespace.as_ref().map(Item::source),
        "Whitespace expected after tag",
    );

    let whitespace = match (has_whitespace, tag_str) {
        (true, "{_}") => {
            let space = if leading_whitespace.is_some()
                || matches!(&trailing_whitespace, Some(Item::Whitespace(Static(" ", _))))
            {
                " "
            } else {
                ""
            };
            vec![Item::Whitespace(Static(space, source))]
        }
        (false, "{_}") => vec![Item::CompileError {
            message: "Whitespace replace tag `{_}` used between non-whitespace. Either add \
                      whitespace or remove this tag."
                .to_string(),
            error_source: source.clone(),
            consumed_source: source,
        }],
        // Return the tag as a comment to keep contiguous source
        // without actually outputting anything.
        (_, "{-}") => {
            vec![Item::Whitespace(Static("", source))]
        }
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

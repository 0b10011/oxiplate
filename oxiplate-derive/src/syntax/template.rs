use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{eof, opt};
use nom::multi::many0;
use nom::{Input, Parser as _};
use nom_language::error::{VerboseError, VerboseErrorKind};
use proc_macro2::{LineColumn, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};

use super::super::Source;
use super::item::{parse_tag, ItemToken};
use super::r#static::{parse_static, StaticType};
use super::{Item, Res, Static};
use crate::syntax::statement::StatementKind;
use crate::State;

/// Collection of items in the template and estimated output length.
#[derive(Debug)]
pub(crate) struct Template<'a>(pub(crate) Vec<Item<'a>>);

impl Template<'_> {
    /// Estimated length of the output of the template.
    #[inline]
    pub(crate) fn estimated_length(&self) -> usize {
        let mut length = 0;
        for item in &self.0 {
            length += item.estimated_length();
        }
        length
    }

    #[inline]
    fn write_tokens(str_tokens: &mut Vec<TokenStream>, tokens: &mut TokenStream) {
        if str_tokens.is_empty() {
            return;
        }

        let concat_tokens = quote! { concat!(#(#str_tokens),*) };
        str_tokens.clear();

        tokens.append_all(quote! { f.write_str(#concat_tokens)?; });
    }
}

impl ToTokens for Template<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut str_tokens = vec![];
        for item in &self.0 {
            match item.to_token() {
                ItemToken::Comment => (),
                ItemToken::StaticText(token_stream) => {
                    str_tokens.push(token_stream);
                }
                ItemToken::DynamicText(token_stream) => {
                    // Write out static text
                    if !str_tokens.is_empty() {
                        Self::write_tokens(&mut str_tokens, tokens);
                    }

                    tokens.append_all(quote! {
                        #token_stream;
                    });
                }
                ItemToken::Statement(token_stream) => {
                    Self::write_tokens(&mut str_tokens, tokens);
                    tokens.append_all(token_stream);
                }
            }
        }

        if !str_tokens.is_empty() {
            Self::write_tokens(&mut str_tokens, tokens);
        }
    }
}

pub(crate) fn parse<'a>(state: &'a State<'a>, source: Source<'a>) -> (TokenStream, usize) {
    match try_parse(state, source) {
        Ok((_, template)) => template,
        Err(
            nom::Err::Error(VerboseError { errors }) | nom::Err::Failure(VerboseError { errors }),
        ) => {
            let template = Template(vec![convert_error(errors)]);
            (quote! { #template }, 0)
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

fn try_parse<'a>(
    state: &'a State<'a>,
    source: Source<'a>,
) -> Res<Source<'a>, (TokenStream, usize)> {
    let (input, items_vec) = many0(parse_item(state, &false)).parse(source)?;

    // Return error if there's any input remaining.
    // Successful value is `("", "")`, so no need to capture.
    let (input, _) = eof(input)?;

    let mut items = Vec::new();
    for mut item_vec in items_vec {
        items.append(&mut item_vec);
    }

    let mut has_content = false;
    let mut extends = None;
    for item in &items {
        match item {
            Item::Statement(statement) => {
                #[allow(clippy::enum_glob_use)]
                use StatementKind::*;
                match &statement.kind {
                    Extends(item) => {
                        if has_content || extends.is_some() {
                            todo!("Can't extend if already adding content");
                        }

                        extends = Some(item);
                    }
                    Block(_) => {
                        // While blocks are allowed when extending,
                        // the extends tag should cause an error if it appears _after_ a block.
                        has_content = true;
                    }
                    Parent | EndBlock | Include(_) | If(_) | ElseIf(_) | Else | EndIf | For(_)
                    | EndFor => {
                        if extends.is_some() {
                            todo!("Can't add content if extending");
                        }

                        has_content = true;
                    }
                }
            }
            #[allow(clippy::match_same_arms)]
            Item::Writ(_) => (),
            Item::Static(_, static_type) => {
                if extends.is_some() {
                    todo!("Can't add static content or writs when extending");
                }

                has_content = static_type == &StaticType::Whitespace;
            }
            // These are fine anywhere
            Item::CompileError(_, _) | Item::Comment | Item::Whitespace(_) => (),
        }
    }

    let template: (TokenStream, usize) = if let Some(extends) = extends {
        extends.build_template()
    } else {
        let template = Template(items);
        (quote! { #template }, template.estimated_length())
    };

    Ok((input, template))
}

pub(crate) fn parse_item<'a>(
    state: &'a State,
    is_extending: &'a bool,
) -> impl Fn(Source) -> Res<Source, Vec<Item>> + 'a {
    |input| {
        alt((
            parse_tag(state, is_extending),
            parse_static,
            adjusted_whitespace,
        ))
        .parse(input)
    }
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

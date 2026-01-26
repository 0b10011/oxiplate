use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{eof, opt};
use nom::multi::many0;
use nom_language::error::{VerboseError, VerboseErrorKind};
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote};

use super::super::Source;
use super::item::{ItemToken, parse_tag};
use super::r#static::parse_static;
use super::{Item, Res, Static};
use crate::syntax::item::{WhitespacePreference, parse_trailing_whitespace};
use crate::{BuiltTokens, State, internal_error};

/// Collection of items in the template and estimated output length.
#[derive(Debug)]
pub(crate) struct Template<'a>(pub(crate) Vec<Item<'a>>);

impl<'a> Template<'a> {
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

    pub fn to_tokens<'b: 'a>(&'a self, state: &mut State<'b>) -> BuiltTokens {
        let mut tokens = TokenStream::new();
        let mut estimated_length = 0;

        let mut str_tokens = vec![];
        for item in &self.0 {
            match item.to_token(state) {
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

pub(crate) fn parse<'a, 'b: 'a>(state: &mut State<'b>, source: Source<'a>) -> BuiltTokens {
    match try_parse(state, source) {
        Ok((_, template)) => template,
        Err(
            nom::Err::Error(VerboseError { errors }) | nom::Err::Failure(VerboseError { errors }),
        ) => {
            let template = Template(convert_error(errors));
            template.to_tokens(state)
        }
        Err(nom::Err::Incomplete(_)) => {
            unreachable!("This should only happen in nom streams which aren't used by Oxiplate.")
        }
    }
}

fn convert_error(errors: Vec<(Source, VerboseErrorKind)>) -> Vec<Item> {
    let mut items: Vec<Item> = vec![];
    for (source, kind) in errors {
        match kind {
            VerboseErrorKind::Char(expected_char) => {
                items.push(Item::CompileError {
                    message: format!("Expected `{expected_char}`"),
                    error_source: source.clone(),
                    consumed_source: source,
                });
            }
            VerboseErrorKind::Context(error) => {
                return vec![Item::CompileError {
                    message: error.to_string(),
                    error_source: source.clone(),
                    consumed_source: source,
                }];
            }
            VerboseErrorKind::Nom(nom_error) => {
                items.push(Item::CompileError {
                    message: format!("Error when parsing with `{nom_error:?}`"),
                    error_source: source.clone(),
                    consumed_source: source,
                });
            }
        }
    }

    items
}

fn try_parse<'a, 'b: 'a>(
    state: &mut State<'b>,
    source: Source<'a>,
) -> Res<Source<'a>, BuiltTokens> {
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
            internal_error!(tag.span().unwrap(), "Unhandled whitespace adjustment tag");
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

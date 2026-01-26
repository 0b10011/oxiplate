use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote};

use super::item::{ItemToken, parse_tag};
use super::r#static::parse_static;
use super::{Item, Res, Static};
#[cfg(coverage_nightly)]
use crate::Source;
use crate::syntax::item::parse_trailing_whitespace;
use crate::syntax::parser::{Parser as _, alt, opt, parse_all, take};
use crate::tokenizer::{TokenKind, TokenSlice, WhitespacePreference};
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

    pub fn to_tokens<'b: 'a>(&'a self, state: &mut State<'b>) -> (TokenStream, usize) {
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

pub(crate) fn parse<'a, 'b: 'a>(
    state: &mut State<'b>,
    tokens: TokenSlice<'a>,
    output_tokens: bool,
) -> BuiltTokens {
    match try_parse(state, tokens, output_tokens) {
        Ok((_, template)) => template,
        Err(error) => Into::<Template>::into(error).to_tokens(state),
    }
}

pub fn try_parse<'a, 'b: 'a>(
    state: &mut State<'b>,
    tokens: TokenSlice<'a>,
    output_tokens: bool,
) -> Res<'a, BuiltTokens> {
    if output_tokens {
        panic!("{tokens:#?}");
    }

    let (tokens, items_vec) = parse_all(parse_item).parse(tokens)?;

    let mut items = Vec::new();
    for mut item_vec in items_vec {
        items.append(&mut item_vec);
    }

    let template = Template(items);

    // Ensure all tested items build source properly.
    #[cfg(coverage_nightly)]
    let _ = template.source();

    Ok((tokens, template.to_tokens(state)))
}

pub(crate) fn parse_item(tokens: TokenSlice) -> Res<Vec<Item>> {
    alt((parse_tag, adjusted_whitespace, parse_static)).parse(tokens)
}

pub(crate) fn adjusted_whitespace(tokens: TokenSlice) -> Res<Vec<Item>> {
    let (tokens, (leading_whitespace, tag)) = (
        opt(take(TokenKind::StaticWhitespace)),
        alt((
            take(TokenKind::WhitespaceAdjustmentTag {
                whitespace_preference: WhitespacePreference::Remove,
            }),
            take(TokenKind::WhitespaceAdjustmentTag {
                whitespace_preference: WhitespacePreference::Replace,
            }),
        )),
    )
        .parse(tokens)?;

    let TokenKind::WhitespaceAdjustmentTag {
        whitespace_preference,
    } = tag.kind()
    else {
        internal_error!(
            tag.source().span_token().unwrap(),
            "Unhandled whitespace adjustment tag"
        );
    };

    let (tokens, trailing_whitespace) = parse_trailing_whitespace(
        tag.source(),
        whitespace_preference,
        leading_whitespace.is_some(),
    )
    .parse(tokens)?;

    let has_whitespace =
        leading_whitespace.is_some() || matches!(&trailing_whitespace, Some(Item::Whitespace(_)));

    let source = tag
        .source()
        .clone()
        .append_to_some(
            leading_whitespace.map(|token| token.source().clone()),
            "Whitespace adjustment tag expected after whitespace",
        )
        .merge_some(
            trailing_whitespace.as_ref().map(Item::source),
            "Whitespace expected after tag",
        );

    let whitespace = match (has_whitespace, whitespace_preference) {
        (true, WhitespacePreference::Replace) => {
            let space = if leading_whitespace.is_some()
                || matches!(&trailing_whitespace, Some(Item::Whitespace(Static(" ", _))))
            {
                " "
            } else {
                ""
            };
            vec![Item::Whitespace(Static(space, source))]
        }
        (false, WhitespacePreference::Replace) => vec![Item::CompileError {
            message: "Whitespace replace tag `{_}` used between non-whitespace. Either add \
                      whitespace or remove this tag."
                .to_string(),
            error_source: source.clone(),
            consumed_source: source,
        }],
        // Return the tag as a comment to keep contiguous source
        // without actually outputting anything.
        (_, WhitespacePreference::Remove) => {
            vec![Item::Whitespace(Static("", source))]
        }
        _ => unreachable!("Only whitespace control tags should be matched"),
    };

    Ok((tokens, whitespace))
}

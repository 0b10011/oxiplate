use std::collections::HashSet;

use super::expression::{ident, Identifier};
use super::{
    expression::expression, item::tag_end, template::is_whitespace, Expression, Item, Res, Static,
};
use crate::Source;
use nom::combinator::{cut, fail};
use nom::error::{context, VerboseError};
use nom::sequence::{preceded, tuple};
use nom::{bytes::complete::take_while, character::complete::char, combinator::opt};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Writ<'a>(pub Expression<'a>, HtmlEscaper);

impl<'a> From<Writ<'a>> for Item<'a> {
    fn from(writ: Writ<'a>) -> Self {
        Item::Writ(writ)
    }
}

impl ToTokens for Writ<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.1.escape(&self.0, tokens);
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum HtmlEscaper {
    #[default]
    Text,
    Attr,
    Comment,
    Raw,
}

impl HtmlEscaper {
    pub fn build(ident: Identifier) -> Result<HtmlEscaper, nom::Err<VerboseError<Source>>> {
        match ident.0 {
            "text" => Ok(HtmlEscaper::Text),
            "attr" => Ok(HtmlEscaper::Attr),
            "comment" => Ok(HtmlEscaper::Comment),
            "raw" => Ok(HtmlEscaper::Raw),
            _ => {
                context("Invalid escaper", fail::<_, (), _>)(ident.1)?;
                unreachable!("fail() should always bail early");
            }
        }
    }

    pub fn escape(&self, expression: &Expression, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            HtmlEscaper::Text => quote! {
                write!(f, "{}", format!("{}", #expression).chars().map(|character| match character {
                    '&' => format!("&amp;"),
                    '<' => format!("&lt;"),
                    _ => format!("{}", character),
                }).collect::<String>())?;
            },
            HtmlEscaper::Attr => quote! {
                write!(f, "{}", format!("{}", #expression).chars().map(|character| match character {
                    '&' => format!("&amp;"),
                    '<' => format!("&lt;"),
                    '"' => format!("&#34;"),
                    '\'' => format!("&#39;"),
                    _ => format!("{}", character),
                }).collect::<String>())?;
            },
            HtmlEscaper::Comment => quote! {
                // Replace `hyphen-minus` with visually similar `minus`.
                write!(f, "{}", format!("{}", #expression).replace("-->", "−−>"))?;
            },
            HtmlEscaper::Raw => quote! { write!(f, "{}", #expression)?; },
        });
    }
}

pub(super) fn writ<'a>(
    local_variables: &'a HashSet<&'a str>,
) -> impl Fn(Source) -> Res<Source, (Item, Option<Static>)> + 'a {
    |input| {
        let (input, _) = take_while(is_whitespace)(input)?;
        let (input, escaper_info) =
            opt(tuple((ident, char(':'), take_while(is_whitespace))))(input)?;
        let escaper = if let Some((escaper, _colon, _whitespace)) = escaper_info {
            HtmlEscaper::build(escaper)?
        } else {
            HtmlEscaper::default()
        };
        let (input, output) =
            context("Expected an expression.", cut(expression(local_variables)))(input)?;
        let (input, trailing_whitespace) = context(
            "Expecting the writ tag to be closed with `_}}`, `-}}`, or `}}`.",
            cut(preceded(take_while(is_whitespace), cut(tag_end("}}")))),
        )(input)?;

        Ok((input, (Writ(output, escaper).into(), trailing_whitespace)))
    }
}

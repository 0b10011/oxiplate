use super::expression::{ident, Identifier};
use super::{
    expression::expression, item::tag_end, template::is_whitespace, Expression, Item, Res, Static,
};
use crate::{Source, State};
use nom::combinator::{cut, fail};
use nom::error::{context, VerboseError};
use nom::sequence::{preceded, terminated, tuple};
use nom::{bytes::complete::take_while, character::complete::char, combinator::opt};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use std::fmt::Debug;
use syn::token::PathSep;
use syn::{Path, PathSegment};

pub(crate) struct Writ<'a>(pub Expression<'a>, Option<Path>);

impl Debug for Writ<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Writ")
            .field(&self.0)
            .field(&"escaper path is skipped")
            .finish()
    }
}

impl<'a> From<Writ<'a>> for Item<'a> {
    fn from(writ: Writ<'a>) -> Self {
        Item::Writ(writ)
    }
}

impl ToTokens for Writ<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let text = &self.0;

        if self.1.is_none() {
            return tokens.append_all(quote! {
                write!(f, "{}", #text)?;
            });
        }

        let escaper = &self.1;

        tokens.append_all(quote! {
            write!(f, "{}", ::oxiplate::escape(#escaper, #text))?;
        });
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Escaper;

impl Escaper {
    pub fn build<'a, 'b>(
        state: &'b State<'b>,
        group: Option<Identifier<'a>>,
        escaper: Identifier<'a>,
    ) -> Result<Option<Path>, nom::Err<VerboseError<Source<'a>>>> {
        let group = if let Some(group) = group {
            Some((group.0, group.1))
        } else if let Some(default_group) = &state.config.default_escaper_group {
            Some((default_group.as_str(), escaper.1.clone()))
        } else {
            None
        };
        let Some(group) = group else {
            if escaper.0 == "raw" {
                return Ok(None);
            }

            context(
                r#"No default escaper group defined and the specified escaper is not "raw""#,
                fail::<_, (), _>,
            )(escaper.1.clone())?;
            unreachable!("fail() should always bail early");
        };

        let Some(group) = state.config.escaper_groups.get(group.0) else {
            context("Invalid escaper group specified", fail::<_, (), _>)(group.1.clone())?;
            unreachable!("fail() should always bail early");
        };

        // Strip underscores and capitalize first character at the beginning and after underscores.
        // That is, `hello_world` becomes `HelloWorld`.
        let mut escaper_variant = String::with_capacity(escaper.0.len());
        let mut capitalize_next = true;
        for char in escaper.0.chars() {
            match (capitalize_next, char) {
                (_, '_') => capitalize_next = true,
                (true, _) => {
                    escaper_variant.push(char.to_ascii_uppercase());
                    capitalize_next = false;
                }
                (_, _) => escaper_variant.push(char),
            }
        }
        if let Ok(escaper) = syn::parse_str::<PathSegment>(&escaper_variant) {
            if let Ok(group) = syn::parse_str::<Path>(&group.escaper) {
                if let Ok(sep) = syn::parse_str::<PathSep>("::") {
                    let path = syn::parse2::<Path>(quote! {
                        #group #sep #escaper
                    });
                    if let Ok(path) = path {
                        return Ok(Some(path));
                    }
                }
            }
        }

        context("Invalid escaper specified", fail::<_, (), _>)(escaper.1)?;
        unreachable!("fail() should always bail early");
    }

    pub fn default(_state: &State) -> Option<Path> {
        None
    }
}

pub(super) fn writ<'a>(
    state: &'a State<'a>,
) -> impl Fn(Source) -> Res<Source, (Item, Option<Static>)> + 'a {
    |input| {
        let (input, _) = take_while(is_whitespace)(input)?;
        let (input, escaper_info) = opt(tuple((
            opt(terminated(ident, char('.'))),
            ident,
            char(':'),
            take_while(is_whitespace),
        )))(input)?;
        let escaper = if let Some((escaper_group, escaper, _colon, _whitespace)) = escaper_info {
            Escaper::build(state, escaper_group, escaper)?
        } else {
            Escaper::default(state)
        };
        let (input, output) = context("Expected an expression.", cut(expression(state)))(input)?;
        let (input, trailing_whitespace) = context(
            "Expecting the writ tag to be closed with `_}}`, `-}}`, or `}}`.",
            cut(preceded(take_while(is_whitespace), cut(tag_end("}}")))),
        )(input)?;

        Ok((input, (Writ(output, escaper).into(), trailing_whitespace)))
    }
}

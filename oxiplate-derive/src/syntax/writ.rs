use std::fmt::Debug;

use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::combinator::{cut, fail, opt};
use nom::error::context;
use nom::sequence::{preceded, terminated};
use nom::Parser as _;
use nom_language::error::VerboseError;
use proc_macro2::TokenStream;
use quote::quote;
use syn::token::PathSep;
use syn::{Path, PathSegment};

use super::expression::{expression, ident, ExpressionAccess, Identifier};
use super::item::tag_end;
use super::template::is_whitespace;
use super::{Item, Res};
use crate::{Source, State};

pub(crate) struct Writ<'a>(pub ExpressionAccess<'a>, Escaper);

impl Writ<'_> {
    pub(crate) fn to_token(&self) -> TokenStream {
        let text = &self.0;

        match &self.1 {
            Escaper::Specified(escaper) => {
                quote! { ::oxiplate::escapers::escape(&#escaper, &format!("{}", #text)) }
            }
            Escaper::Default(escaper) => {
                quote! { ::oxiplate::escapers::escape(&<#escaper as ::oxiplate::escapers::Escaper>::DEFAULT, &format!("{}", #text)) }
            }
            Escaper::None => quote! { #text },
        }
    }
}

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

enum Escaper {
    Specified(Path),
    Default(Path),
    None,
}

impl Escaper {
    pub fn build<'a, 'b>(
        state: &'b State<'b>,
        group: Option<Identifier<'a>>,
        escaper: Identifier<'a>,
    ) -> Result<Escaper, nom::Err<VerboseError<Source<'a>>>> {
        if escaper.ident == "raw" {
            return Ok(Escaper::None);
        }

        let group = if let Some(group) = group {
            (group.ident, group.source)
        } else if let Some(default_group) = &state.config.default_escaper_group {
            (default_group.as_str(), escaper.source.clone())
        } else {
            context(
                r#"No default escaper group defined and the specified escaper is not "raw""#,
                fail::<_, (), _>(),
            )
            .parse(escaper.source.clone())?;
            unreachable!("fail() should always bail early");
        };

        let Some(group) = state.config.escaper_groups.get(group.0) else {
            context("Invalid escaper group specified", fail::<_, (), _>())
                .parse(group.1.clone())?;
            unreachable!("fail() should always bail early");
        };

        // Strip underscores and capitalize first character at the beginning and after underscores.
        // That is, `hello_world` becomes `HelloWorld`.
        let mut escaper_variant = String::with_capacity(escaper.ident.len());
        let mut capitalize_next = true;
        for char in escaper.ident.chars() {
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
                        return Ok(Escaper::Specified(path));
                    }
                }
            }
        }

        context("Invalid escaper specified", fail::<_, (), _>()).parse(escaper.source)?;
        unreachable!("fail() should always bail early");
    }

    pub fn default<'a>(
        state: &State,
        input: &Source<'a>,
    ) -> Result<Escaper, nom::Err<VerboseError<Source<'a>>>> {
        let Some(default_group) = &state.config.default_escaper_group else {
            context(
                r#"No default escaper group defined and no escaper was specified. If escaping is not wanted in ANY files, set `default_escaper_group = "raw"` in `/oxiplate.toml`. If escaping is not wanted just in this one instance, prefix the writ with `raw:`."#,
                fail::<_, (), _>(),
            )
            .parse(input.clone())?;
            unreachable!("fail() should always bail early");
        };

        if default_group == "raw" {
            return Ok(Escaper::None);
        }

        let Some(default_group) = state.config.escaper_groups.get(default_group) else {
            context(
                "Invalid default escaper group specified",
                fail::<_, (), _>(),
            )
            .parse(input.clone())?;
            unreachable!("fail() should always bail early");
        };

        let Ok(group) = syn::parse_str::<Path>(&default_group.escaper) else {
            context("Unparseable default escaper group path", fail::<_, (), _>())
                .parse(input.clone())?;
            unreachable!("fail() should always bail early");
        };

        Ok(Escaper::Default(group))
    }
}

pub(super) fn writ<'a>(
    state: &'a State<'a>,
) -> impl Fn(Source) -> Res<Source, (Item, Option<Item>)> + 'a {
    |input| {
        let (input, _) = take_while(is_whitespace)(input)?;

        let (input, escaper_info) = opt((
            opt(terminated(ident, char('.'))),
            ident,
            char(':'),
            take_while(is_whitespace),
        ))
        .parse(input)?;
        let escaper = if let Some((escaper_group, escaper, _colon, _whitespace)) = escaper_info {
            Escaper::build(state, escaper_group, escaper)?
        } else {
            Escaper::default(state, &input)?
        };

        let (input, output) = context(
            "Expected an expression.",
            cut(expression(state, true, true)),
        )
        .parse(input)?;
        let (input, trailing_whitespace) = context(
            "Expecting the writ tag to be closed with `_}}`, `-}}`, or `}}`.",
            cut(preceded(take_while(is_whitespace), cut(tag_end("}}")))),
        )
        .parse(input)?;

        Ok((input, (Writ(output, escaper).into(), trailing_whitespace)))
    }
}

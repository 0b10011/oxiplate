use std::fmt::Debug;

use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::combinator::{cut, fail, opt};
use nom::error::context;
use nom::sequence::{preceded, terminated};
use nom::Parser as _;
use nom_language::error::VerboseError;
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;
#[cfg(feature = "oxiplate")]
use syn::token::PathSep;
#[cfg(feature = "oxiplate")]
use syn::Path;
#[cfg(feature = "oxiplate")]
use syn::PathSegment;

use super::expression::{expression, ident, ExpressionAccess, Identifier};
use super::item::tag_end;
use super::template::is_whitespace;
use super::{Item, Res};
use crate::{Source, State};

pub(crate) struct Writ<'a>(pub ExpressionAccess<'a>, Escaper);

impl Writ<'_> {
    pub(crate) fn to_token(&self) -> TokenStream {
        let text = &self.0;

        // Errors with the escape/raw calls
        // will almost always be issues
        // with the type of the expression being escaped,
        // so it'll be more helpful to use the expression's span
        // rather than the escaper's.
        let span = self.0.span();

        match &self.1 {
            #[cfg(feature = "oxiplate")]
            Escaper::Specified(escaper) => {
                quote_spanned! {span=>
                    (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&#text)).oxiplate_escape(
                        f,
                        &#escaper,
                    )?
                }
            }
            #[cfg(feature = "oxiplate")]
            Escaper::Default(escaper) => {
                quote_spanned! {span=>
                    (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&#text)).oxiplate_escape(
                        f,
                        &<#escaper as ::oxiplate::escapers::Escaper>::DEFAULT,
                    )?
                }
            }
            #[cfg(feature = "oxiplate")]
            Escaper::None => quote_spanned! {span=>
                (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&#text)).oxiplate_raw(f)?
            },
            #[cfg(not(feature = "oxiplate"))]
            Escaper::None => quote_spanned! {span=>
                f.write_str(&::std::string::ToString::to_string(&#text))?;
            },
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
    #[cfg(feature = "oxiplate")]
    Specified(Path),
    #[cfg(feature = "oxiplate")]
    Default(Path),
    None,
}

impl Escaper {
    #[cfg(not(feature = "oxiplate"))]
    pub fn build(escaper: Identifier<'_>) -> Result<Escaper, nom::Err<VerboseError<Source<'_>>>> {
        if escaper.ident != "raw" {
            context(
                "Escaper other than `raw` specified, but `oxiplate` is not available for escaping.",
                fail::<_, (), _>(),
            )
            .parse(escaper.source)?;
            unreachable!("fail() should always bail early");
        }

        Ok(Escaper::None)
    }

    #[cfg(feature = "oxiplate")]
    pub fn build<'a, 'b>(
        state: &'b State<'b>,
        group: Option<Identifier<'a>>,
        escaper: Identifier<'a>,
    ) -> Result<Escaper, nom::Err<VerboseError<Source<'a>>>> {
        if escaper.ident == "raw" {
            return Ok(Escaper::None);
        }

        let escaper_group = if let Some(group) = group {
            let Some(escaper_group) = state.config.escaper_groups.get(group.ident) else {
                context("Invalid escaper group specified", fail::<_, (), _>())
                    .parse(group.source.clone())?;
                unreachable!("fail() should always bail early");
            };

            (escaper_group, group.source)
        } else if let Some(inferred_group) = state.inferred_escaper_group {
            (inferred_group, escaper.source.clone())
        } else if let Some(fallback_group) = &state.config.fallback_escaper_group {
            let Some(escaper_group) = state.config.escaper_groups.get(fallback_group.as_str())
            else {
                context(
                    "Invalid fallback escaper group specified",
                    fail::<_, (), _>(),
                )
                .parse(escaper.source.clone())?;
                unreachable!("fail() should always bail early");
            };

            (escaper_group, escaper.source.clone())
        } else {
            #[cfg(not(feature = "config"))]
            context(
                r#"An escaper other than "raw" is specified, but the `config` feature is turned off, so no default escaper group could be defined."#,
                fail::<_, (), _>(),
            )
            .parse(escaper.source)?;

            #[cfg(feature = "config")]
            context(
                r#"No default escaper group defined and the specified escaper is not "raw". Consider setting a value for `fallback_escaper_group` in `/oxiplate.toml`, or turn on the `built-in-escapers` Oxiplate feature."#,
                fail::<_, (), _>(),
            )
            .parse(escaper.source)?;
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

        if let Ok(escaper) =
            syn::LitStr::new(&escaper_variant, escaper.span()).parse::<PathSegment>()
        {
            if let Ok(group) =
                syn::LitStr::new(&escaper_group.0.escaper, escaper_group.1.span()).parse::<Path>()
            {
                if let Ok(sep) = syn::LitStr::new("::", escaper.span()).parse::<PathSep>() {
                    let span = escaper.span();
                    let path = syn::parse2::<Path>(quote_spanned! {span=>
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

    #[cfg(not(feature = "oxiplate"))]
    pub fn default<'a>(
        state: &State,
        input: &Source<'a>,
    ) -> Result<Escaper, nom::Err<VerboseError<Source<'a>>>> {
        if state.config.require_specifying_escaper {
            context(
                r"Escapers must be specified on all writs due to `require_specifying_escaper` config setting being set to `true` in `/oxiplate.toml`.",
                fail::<_, (), _>(),
            )
            .parse(input.clone())?;
        } else if state.config.fallback_escaper_group != Some("raw".to_string()) {
            context(
                "Default escapers are only possible when using `oxiplate` rather than \
                 `oxiplate-derive` directly.",
                fail::<_, (), _>(),
            )
            .parse(input.clone())?;
            unreachable!("fail() should always bail early");
        }

        Ok(Escaper::None)
    }

    #[cfg(feature = "oxiplate")]
    pub fn default<'a>(
        state: &State,
        input: &Source<'a>,
    ) -> Result<Escaper, nom::Err<VerboseError<Source<'a>>>> {
        if state.config.require_specifying_escaper {
            context(
                r"Escapers must be specified on all writs due to `require_specifying_escaper` config setting being set to `true` in `/oxiplate.toml`.",
                fail::<_, (), _>(),
            )
            .parse(input.clone())?;
        }

        let default_group = if let Some(inferred_group) = state.inferred_escaper_group {
            inferred_group
        } else if let Some(fallback_group) = &state.config.fallback_escaper_group {
            if fallback_group == "raw" {
                return Ok(Escaper::None);
            }

            let Some(fallback_group) = state.config.escaper_groups.get(fallback_group) else {
                context(
                    "Invalid default escaper group specified. Make sure the escaper name in the \
                     template matches the name set in `/oxiplate.toml`.",
                    fail::<_, (), _>(),
                )
                .parse(input.clone())?;
                unreachable!("fail() should always bail early");
            };

            fallback_group
        } else {
            #[cfg(not(feature = "config"))]
            context(
                r#"No escaper is specified, it could not be inferred from the template's file extension, and the "config" feature is turned off so no default escaper group could be defined. Check to make sure the template's file extension is correct."#,
                fail::<_, (), _>(),
            )
            .parse(input.clone())?;

            #[cfg(feature = "config")]
            context(
                r#"No escaper is specified, it could not be inferred from the template's file extension, and there was also no fallback escaper group defined. Check to make sure the template's file extension is correct. If escaping is not wanted in ANY files, set `fallback_escaper_group = "raw"` in `/oxiplate.toml`. If escaping is not wanted just in this one instance, prefix the writ with `raw:`."#,
                fail::<_, (), _>(),
            )
            .parse(input.clone())?;

            unreachable!("fail() should always bail early");
        };

        let Ok(group) = syn::LitStr::new(&default_group.escaper, input.span()).parse::<Path>()
        else {
            context(
                r#"Unparseable default escaper group path. Make sure the escaper path is correct in \
                `/oxiplate.toml`. It should look something like `escaper = "::oxiplate::escapers::html::HtmlEscaper"`."#,
                fail::<_, (), _>(),
            )
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

        #[cfg_attr(not(feature = "oxiplate"), allow(unused_variables))]
        let escaper = if let Some((escaper_group, escaper, _colon, _whitespace)) = escaper_info {
            Escaper::build(
                #[cfg(feature = "oxiplate")]
                state,
                #[cfg(feature = "oxiplate")]
                escaper_group,
                escaper,
            )?
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

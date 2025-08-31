use std::fmt::Debug;

use nom::Parser as _;
use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::combinator::{cut, opt};
use nom::error::context;
use nom::sequence::{preceded, terminated};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::token::PathSep;
use syn::{Path, PathSegment};

use super::expression::{ExpressionAccess, Identifier, expression, ident};
use super::item::tag_end;
use super::template::is_whitespace;
use super::{Item, Res};
use crate::state::EscaperGroup;
use crate::{Source, State};

enum EscaperType<'a> {
    Default,
    Specified((&'a str, &'a EscaperGroup), Span, &'a Identifier<'a>),
    Raw,
}

macro_rules! token_error {
    ($span:ident, $message:literal $(,)?) => {
        (quote_spanned! {$span=> compile_error!($message); }, 0)
    };
}

/// Expression to output and the specified escaper.
/// If no escaper is provided,
/// the _default_ escaper is assumed.
pub(crate) struct Writ<'a>(pub ExpressionAccess<'a>, Option<Escaper<'a>>);

impl Writ<'_> {
    pub(crate) fn to_token(&self, state: &State<'_>) -> (TokenStream, usize) {
        let mut estimated_length = 0;

        let (text, text_length) = &self.0.to_tokens(state);
        estimated_length += text_length;

        // Errors with the escape/raw calls
        // will almost always be issues
        // with the type of the expression being escaped,
        // so it'll be more helpful to use the expression's span
        // rather than the escaper's.
        let span = text.span();

        let escaper_type: EscaperType = match self.escaper_type(state, text) {
            Ok(escaper_type) => escaper_type,
            Err(tokens) => return tokens,
        };

        match escaper_type {
            EscaperType::Default => Self::escaper_default(state, span, text, estimated_length),
            EscaperType::Specified(group, group_span, escaper) => Self::escaper_specified(
                state,
                group,
                group_span,
                escaper,
                span,
                text,
                estimated_length,
            ),
            EscaperType::Raw => Self::escaper_raw(text, estimated_length),
        }
    }

    fn escaper_type<'a>(
        &'a self,
        state: &'a State,
        text: &TokenStream,
    ) -> Result<EscaperType<'a>, (TokenStream, usize)> {
        match &self.1 {
            Some(Escaper {
                group: Some(group),
                escaper,
            }) => {
                if let Some(escaper_group) = state.config.escaper_groups.get(group.ident) {
                    Ok(EscaperType::Specified(
                        (group.ident, escaper_group),
                        group.source.span(),
                        escaper,
                    ))
                } else {
                    let span = group.span();
                    Err((
                        quote_spanned! {span=>
                            compile_error!("Invalid escaper group specified");
                        },
                        0,
                    ))
                }
            }
            Some(Escaper {
                group: None,
                escaper,
            }) => {
                if escaper.ident == "raw" {
                    Ok(EscaperType::Raw)
                } else if let Some(default_group) = state.default_escaper_group {
                    Ok(EscaperType::Specified(
                        default_group,
                        escaper.source.span(),
                        escaper,
                    ))
                } else if let Some(inferred_group) = state.inferred_escaper_group {
                    Ok(EscaperType::Specified(
                        inferred_group,
                        escaper.source.span(),
                        escaper,
                    ))
                } else if let Some(fallback_group) = &state.config.fallback_escaper_group {
                    if let Some(escaper_group) =
                        state.config.escaper_groups.get(fallback_group.as_str())
                    {
                        Ok(EscaperType::Specified(
                            (fallback_group.as_str(), escaper_group),
                            escaper.source.span(),
                            escaper,
                        ))
                    } else {
                        let span = text.span();
                        Err((
                            quote_spanned! {span=>
                                compile_error!("Invalid fallback escaper group specified");
                            },
                            0,
                        ))
                    }
                } else {
                    let span = escaper.span();

                    #[cfg(not(feature = "config"))]
                    return Err((
                        quote_spanned! {span=>
                            compile_error!(
                                r#"An escaper other than "raw" is specified, but the `config` feature is turned off, so no escaper groups are defined that might otherwise match."#
                            );
                        },
                        0,
                    ));

                    #[cfg(all(feature = "config", feature = "built-in-escapers"))]
                    return Err((
                        quote_spanned! {span=>
                            compile_error!(
                                r#"No escaper group was selected and the specified escaper is not "raw". Consider setting a value for `fallback_escaper_group` in `/oxiplate.toml`."#
                            );
                        },
                        0,
                    ));

                    #[cfg(all(feature = "config", not(feature = "built-in-escapers")))]
                    return Err((
                        quote_spanned! {span=>
                            compile_error!(
                                r#"No fallback escaper group defined and the specified escaper is not "raw". Consider setting a value for `fallback_escaper_group` in `/oxiplate.toml`, or turn on the `built-in-escapers` Oxiplate feature."#,
                            );
                        },
                        0,
                    ));
                }
            }
            None => Ok(EscaperType::Default),
        }
    }

    fn escaper_default(
        state: &State,
        span: Span,
        text: &TokenStream,
        estimated_length: usize,
    ) -> (TokenStream, usize) {
        if state.config.require_specifying_escaper {
            return token_error!(
                span,
                r"Escapers must be specified on all writs due to `require_specifying_escaper` config setting being set to `true` in `/oxiplate.toml`."
            );
        }

        let default_group: (&str, &EscaperGroup) = if let Some(default_group) =
            state.default_escaper_group
        {
            default_group
        } else if let Some(inferred_group) = state.inferred_escaper_group {
            inferred_group
        } else if let Some(fallback_group_name) = &state.config.fallback_escaper_group {
            if fallback_group_name == "raw" {
                #[cfg(not(feature = "oxiplate"))]
                return (
                    quote_spanned! {span=> f.write_str(&::std::string::ToString::to_string(&(#text)))?; },
                    estimated_length,
                );

                #[cfg(feature = "oxiplate")]
                return (
                    quote_spanned! {span=>
                        (&&::oxiplate::UnescapedTextWrapper::new(&(#text))).oxiplate_raw(f)?
                    },
                    estimated_length,
                );
            }

            let Some(fallback_group) = state.config.escaper_groups.get(fallback_group_name) else {
                return token_error!(
                    span,
                    "Invalid fallback escaper group specified. Make sure the escaper name in the \
                     template matches the name set in `/oxiplate.toml`.",
                );
            };

            (fallback_group_name, fallback_group)
        } else {
            if *state.failed_to_set_default_escaper_group {
                return (
                    quote! { compile_error!("Some writ tokens were not generated due to an error setting the default escaper group."); },
                    0,
                );
            }

            #[cfg(not(feature = "config"))]
            return token_error!(
                span,
                r#"No escaper is specified, it could not be inferred from the template's file extension, and the "config" feature is turned off so no default escaper group could be defined. Check to make sure the template's file extension is correct."#,
            );

            #[cfg(feature = "config")]
            return token_error!(
                span,
                r#"No escaper is specified, it could not be inferred from the template's file extension, and there was also no fallback escaper group defined. Check to make sure the template's file extension is correct. If escaping is not wanted in ANY files, set `fallback_escaper_group = "raw"` in `/oxiplate.toml`. If escaping is not wanted just in this one instance, prefix the writ with `raw:`."#,
            );
        };

        let Ok(group) = syn::LitStr::new(&default_group.1.escaper, span).parse::<Path>() else {
            return token_error!(
                span,
                r#"Unparseable default escaper group path. Make sure the escaper path is correct in \
                            `/oxiplate.toml`. It should look something like `escaper = "::oxiplate::escapers::html::HtmlEscaper"`."#,
            );
        };

        (
            quote_spanned! {span=>
                (&&::oxiplate::UnescapedTextWrapper::new(&(#text))).oxiplate_escape(
                    f,
                    &<#group as ::oxiplate::Escaper>::DEFAULT,
                )?
            },
            estimated_length,
        )
    }

    fn escaper_specified(
        state: &State,
        group: (&str, &EscaperGroup),
        group_span: Span,
        escaper: &Identifier,
        span: Span,
        text: &TokenStream,
        estimated_length: usize,
    ) -> (TokenStream, usize) {
        if *state.failed_to_set_default_escaper_group {
            return (
                quote! { compile_error!("Some writ tokens were not generated due to an error setting the default escaper group."); },
                0,
            );
        }

        if let Ok(escaper) = syn::LitStr::new(escaper.ident, escaper.span()).parse::<PathSegment>()
            && let Ok(group) = syn::LitStr::new(&group.1.escaper, group_span).parse::<Path>()
            && let Ok(sep) = syn::LitStr::new("::", group_span).parse::<PathSep>()
        {
            let path = syn::parse2::<Path>(quote! {
                #group #sep #escaper
            });
            if let Ok(path) = path {
                return (
                    quote_spanned! {span=>
                        (&&::oxiplate::UnescapedTextWrapper::new(&(#text))).oxiplate_escape(
                            f,
                            &#path,
                        )?
                    },
                    estimated_length,
                );
            }
        }

        token_error!(span, r"Failed to build escape function call")
    }

    fn escaper_raw(text: &TokenStream, estimated_length: usize) -> (TokenStream, usize) {
        let span = text.span();

        #[cfg(not(feature = "oxiplate"))]
        return (
            quote_spanned! {span=>
                f.write_str(&::std::string::ToString::to_string(&(#text)))?;
            },
            estimated_length,
        );

        #[cfg(feature = "oxiplate")]
        return (
            quote_spanned! {span=>
                (&&::oxiplate::UnescapedTextWrapper::new(&(#text))).oxiplate_raw(f)?
            },
            estimated_length,
        );
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

struct Escaper<'a> {
    group: Option<Identifier<'a>>,
    escaper: Identifier<'a>,
}

pub(super) fn writ(input: Source) -> Res<Source, (Item, Option<Item>)> {
    let (input, _) = take_while(is_whitespace)(input)?;

    let (input, escaper_info) = opt((
        opt(terminated(ident, char('.'))),
        ident,
        char(':'),
        take_while(is_whitespace),
    ))
    .parse(input)?;

    #[cfg_attr(not(feature = "oxiplate"), allow(unused_variables))]
    let escaper = escaper_info.map(|(escaper_group, escaper, _colon, _whitespace)| Escaper {
        group: escaper_group,
        escaper,
    });

    let (input, output) =
        context("Expected an expression.", cut(expression(true, true))).parse(input)?;
    let (input, trailing_whitespace) = context(
        "Expecting the writ tag to be closed with `_}}`, `-}}`, or `}}`.",
        cut(preceded(take_while(is_whitespace), cut(tag_end("}}")))),
    )
    .parse(input)?;

    Ok((input, (Writ(output, escaper).into(), trailing_whitespace)))
}

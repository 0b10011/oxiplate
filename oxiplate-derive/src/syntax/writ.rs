use std::fmt::Debug;

use nom::Parser as _;
use nom::bytes::complete::{tag, take_while};
use nom::combinator::{cut, opt};
use nom::error::context;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::token::PathSep;
use syn::{Path, PathSegment};

use super::expression::{ExpressionAccess, Identifier, expression};
use super::item::tag_end;
use super::template::is_whitespace;
use super::{Item, Res};
use crate::state::EscaperGroup;
use crate::{Source, State, Tokens};

enum EscaperType<'a> {
    Default,
    Specified((String, &'a EscaperGroup), Span, &'a Identifier<'a>),
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
#[derive(Debug)]
pub(crate) struct Writ<'a> {
    escaper: Option<Escaper<'a>>,
    expression: ExpressionAccess<'a>,
    source: Source<'a>,
}

impl<'a> Writ<'a> {
    pub(crate) fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub(crate) fn to_token(&self, state: &State<'_>) -> Tokens {
        let mut estimated_length = 0;

        let (text, text_length) = &self.expression.to_tokens(state);
        estimated_length += text_length;

        let span = self.source.span();

        let escaper_type: EscaperType = match self.escaper_type(state) {
            Ok(escaper_type) => escaper_type,
            Err(tokens) => return tokens,
        };

        match escaper_type {
            EscaperType::Default => Self::escaper_default(state, span, text, estimated_length),
            EscaperType::Specified(group, group_span, escaper) => Self::escaper_specified(
                state,
                &group,
                group_span,
                escaper,
                span,
                text,
                estimated_length,
            ),
            EscaperType::Raw => Self::escaper_raw(text, estimated_length),
        }
    }

    fn escaper_type(&'a self, state: &'a State) -> Result<EscaperType<'a>, Tokens> {
        match &self.escaper {
            Some(Escaper {
                group: Some(group),
                escaper,
            }) => {
                if let Some(escaper_group) = state.config.escaper_groups.get(group.as_str()) {
                    Ok(EscaperType::Specified(
                        (group.as_str().to_owned(), escaper_group),
                        group.source().span(),
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
                if escaper.as_str() == "raw" {
                    Ok(EscaperType::Raw)
                } else if let Some((name, group)) = &state.default_escaper_group {
                    Ok(EscaperType::Specified(
                        (name.clone(), group),
                        escaper.source().span(),
                        escaper,
                    ))
                } else if state.failed_to_set_default_escaper_group {
                    Err((
                        quote! { compile_error!("Some writ tokens were not generated due to an error setting the default escaper group."); },
                        0,
                    ))
                } else if let Some((name, group)) = &state.inferred_escaper_group {
                    Ok(EscaperType::Specified(
                        (name.clone(), group),
                        escaper.source().span(),
                        escaper,
                    ))
                } else if let Some(fallback_group) = &state.config.fallback_escaper_group {
                    if let Some(escaper_group) =
                        state.config.escaper_groups.get(fallback_group.as_str())
                    {
                        Ok(EscaperType::Specified(
                            (fallback_group.to_owned(), escaper_group),
                            escaper.source().span(),
                            escaper,
                        ))
                    } else {
                        // This should have been caught during initial parsing of the config,
                        // but leaving a helpful error message just in case.
                        let span = escaper.source().span();
                        let fallback_group = fallback_group.as_str();
                        Err((
                            quote_spanned! {span=>
                                compile_error!(concat!("Escaper could not be found because an invalid fallback escaper group `", #fallback_group, "` was specified in `/oxiplate.toml`. Specify the escaper group in the writ, or fix the fallback escaper group in `/oxiplate.toml`."));
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
    ) -> Tokens {
        if state.config.require_specifying_escaper {
            return token_error!(
                span,
                r"Escapers must be specified on all writs due to `require_specifying_escaper` config setting being set to `true` in `/oxiplate.toml`."
            );
        } else if state.failed_to_set_default_escaper_group {
            return (
                quote! { compile_error!("Some writ tokens were not generated due to an error setting the default escaper group."); },
                0,
            );
        }

        let default_group: &(String, EscaperGroup) = if let Some(default_group) =
            &state.default_escaper_group
        {
            default_group
        } else if let Some(inferred_group) = &state.inferred_escaper_group {
            inferred_group
        } else if let Some(fallback_group_name) = &state.config.fallback_escaper_group {
            if fallback_group_name == "raw" {
                #[cfg(not(feature = "oxiplate"))]
                return (
                    quote_spanned! {span=> oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(#text)))?; },
                    estimated_length,
                );

                #[cfg(feature = "oxiplate")]
                return (
                    quote_spanned! {span=>
                        (&&::oxiplate::UnescapedTextWrapper::new(&(#text))).oxiplate_raw(oxiplate_formatter)?
                    },
                    estimated_length,
                );
            }

            let Some(fallback_group) = state.config.escaper_groups.get(fallback_group_name) else {
                // This should have been caught during initial parsing of the config,
                // but leaving a helpful error message just in case.
                return token_error!(
                    span,
                    "Invalid fallback escaper group specified. Make sure the escaper name in the \
                     template matches the name set in `/oxiplate.toml`.",
                );
            };

            &(fallback_group_name.clone(), fallback_group.clone())
        } else {
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
                    oxiplate_formatter,
                    &<#group as ::oxiplate::Escaper>::DEFAULT,
                )?
            },
            estimated_length,
        )
    }

    fn escaper_specified(
        state: &State,
        group: &(String, &EscaperGroup),
        group_span: Span,
        escaper: &Identifier,
        span: Span,
        text: &TokenStream,
        estimated_length: usize,
    ) -> Tokens {
        if state.failed_to_set_default_escaper_group {
            return (
                quote! { compile_error!("Some writ tokens were not generated due to an error setting the default escaper group."); },
                0,
            );
        }

        if let Ok(escaper) =
            syn::LitStr::new(escaper.as_str(), escaper.span()).parse::<PathSegment>()
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
                            oxiplate_formatter,
                            &#path,
                        )?
                    },
                    estimated_length,
                );
            }
        }

        token_error!(span, r"Failed to build escape function call")
    }

    fn escaper_raw(text: &TokenStream, estimated_length: usize) -> Tokens {
        let span = text.span();

        #[cfg(not(feature = "oxiplate"))]
        return (
            quote_spanned! {span=>
                oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(#text)))?;
            },
            estimated_length,
        );

        #[cfg(feature = "oxiplate")]
        return (
            quote_spanned! {span=>
                (&&::oxiplate::UnescapedTextWrapper::new(&(#text))).oxiplate_raw(oxiplate_formatter)?
            },
            estimated_length,
        );
    }
}

impl<'a> From<Writ<'a>> for Item<'a> {
    fn from(writ: Writ<'a>) -> Self {
        Item::Writ(writ)
    }
}

#[derive(Debug)]
struct Escaper<'a> {
    group: Option<Identifier<'a>>,
    escaper: Identifier<'a>,
}

pub(super) fn writ<'a>(
    open_tag_source: Source<'a>,
) -> impl Fn(Source<'a>) -> Res<Source<'a>, (Item<'a>, Option<Item<'a>>)> {
    move |input| {
        let (input, leading_whitespace) = take_while(is_whitespace)(input)?;

        let (input, escaper_info) = opt((
            opt((Identifier::parse, tag("."))),
            Identifier::parse,
            tag(":"),
            take_while(is_whitespace),
        ))
        .parse(input)?;

        let escaper_source = escaper_info.as_ref().map(|escaper_info| {
            if let Some((escaper_group, dot)) = &escaper_info.0 {
                escaper_group
                    .source()
                    .clone()
                    .merge(dot, "Dot expected after escaper group")
                    .merge(escaper_info.1.source(), "Escaper name expected after group")
            } else {
                escaper_info.1.source().clone()
            }
            .merge(&escaper_info.2, "Colon expected after escaper name")
            .merge(&escaper_info.3, "Whitespace expected after colon")
        });

        #[cfg_attr(not(feature = "oxiplate"), allow(unused_variables))]
        let escaper = escaper_info.map(|(escaper_group, escaper, _colon, _whitespace)| Escaper {
            group: escaper_group.map(|(escaper_group, _dot)| escaper_group),
            escaper,
        });

        let (input, output) =
            context("Expected an expression.", cut(expression(true, true))).parse(input)?;
        let (input, (whitespace_in_tag, (trailing_whitespace, end_tag))) = context(
            "Expecting the writ tag to be closed with `_}}`, `-}}`, or `}}`.",
            cut((take_while(is_whitespace), cut(tag_end("}}")))),
        )
        .parse(input)?;

        let source = open_tag_source
            .clone()
            .merge(&leading_whitespace, "Whitespace expected after opening tag")
            .merge_some(escaper_source.as_ref(), "Escaper expected after whitespace")
            .merge(&output.source(), "Expression expected after escaper")
            .merge(&whitespace_in_tag, "Whitespace expected after expression")
            .merge(&end_tag, "End tag expected after whitespace");

        Ok((
            input,
            (
                Writ {
                    escaper,
                    expression: output,
                    source,
                }
                .into(),
                trailing_whitespace,
            ),
        ))
    }
}

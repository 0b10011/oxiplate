use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::LitStr;
use syn::spanned::Spanned as _;

use super::super::expression::Identifier;
use super::{Statement, StatementKind};
use crate::syntax::Res;
use crate::syntax::expression::{Keyword, KeywordParser};
use crate::syntax::parser::{Parser as _, alt, cut};
use crate::tokenizer::TokenSlice;
use crate::{BuiltTokens, Source, State, internal_error};

#[derive(Debug)]
pub struct DefaultEscaper<'a> {
    pub(crate) tag: Keyword<'a>,
    pub(crate) escaper: Identifier<'a>,
    pub(crate) can_replace_inferred_escaper: bool,
}

impl<'a> From<DefaultEscaper<'a>> for StatementKind<'a> {
    fn from(statement: DefaultEscaper<'a>) -> Self {
        StatementKind::DefaultEscaper(statement)
    }
}

impl DefaultEscaper<'_> {
    pub(crate) fn to_tokens(
        &self,
        state: &State,
        statement_source: &Source<'_>,
    ) -> Result<BuiltTokens, BuiltTokens> {
        if state.default_escaper_group.is_some() {
            let span = statement_source.span_token();
            let tag = self.tag.source().as_str();
            let tag_span = self.tag.span();
            let tag = quote_spanned! {tag_span=> #tag };
            Err((
                quote_spanned! {span=> compile_error!(concat!("Unexpected '", #tag, "' statement after already setting the default escaper group")); },
                0,
            ))
        } else if state.has_content {
            let span = statement_source.span_token();
            let tag = self.tag.source().as_str();
            let tag_span = self.tag.span();
            let tag = quote_spanned! {tag_span=> #tag };
            Err((
                quote_spanned! {span=> compile_error!(concat!("Unexpected '", #tag, "' statement after content already present in template")); },
                0,
            ))
        } else {
            if !self.can_replace_inferred_escaper {
                if let Some(inferred_escaper_group) = &state.inferred_escaper_group {
                    if inferred_escaper_group.0 != self.escaper.as_str() {
                        let default_escaper =
                            LitStr::new(self.escaper.as_str(), self.escaper.span());
                        let inferred_escaper_group = &inferred_escaper_group.0;
                        let span = self.escaper.source().span_token();
                        Err((
                            quote_spanned! {span=>
                                compile_error!(concat!(
                                    "Setting the default escaper group to `",
                                    #default_escaper,
                                    "` failed due to the inferred escaper group already being set to `",
                                    #inferred_escaper_group,
                                    "`. If this was intentional, consider using `replace_escaper_group` instead."
                                ));
                            },
                            0,
                        ))?;
                    }
                }
            }
            if !state
                .config
                .escaper_groups
                .contains_key(self.escaper.as_str())
            {
                let span = self.escaper.span();
                let default_escaper = LitStr::new(self.escaper.as_str(), span);
                let mut available_escaper_groups = state
                    .config
                    .escaper_groups
                    .keys()
                    .map(|key| &**key)
                    .collect::<Vec<&str>>();
                available_escaper_groups.sort_unstable();
                let available_escaper_groups =
                    LitStr::new(&available_escaper_groups.join(", "), span);
                Err((
                    quote_spanned! {span=>
                       compile_error!(concat!(
                           "No escaper group named `",
                           #default_escaper,
                           "` was found. Available: ",
                           #available_escaper_groups,
                       ));
                    },
                    0,
                ))?;
            }
            Ok((TokenStream::new(), 0))
        }
    }
}

pub(super) fn parse_default_escaper_group(tokens: TokenSlice) -> Res<Statement> {
    let (tokens, tag) = alt((
        KeywordParser::new("default_escaper_group"),
        KeywordParser::new("replace_escaper_group"),
        #[cfg(feature = "unreachable")]
        KeywordParser::new("unreachable_escaper_group"),
    ))
    .parse(tokens)?;

    let (tokens, escaper) =
        cut("Expected an escaper group name", Identifier::parse).parse(tokens)?;

    let can_replace_inferred_escaper = match tag.source().as_str() {
        "default_escaper_group" => false,
        "replace_escaper_group" => true,
        _ => internal_error!(
            tag.source().span_token().unwrap(),
            "Unhandled default escaper group tag"
        ),
    };

    let source = tag
        .source()
        .clone()
        .merge(escaper.source(), "Escaper name expected after whitespace");

    Ok((
        tokens,
        Statement {
            kind: DefaultEscaper {
                tag,
                escaper,
                can_replace_inferred_escaper,
            }
            .into(),
            source,
        },
    ))
}

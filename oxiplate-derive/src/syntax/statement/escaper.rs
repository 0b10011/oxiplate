use nom::Parser as _;
use nom::branch::alt;
use nom::combinator::cut;
use nom::error::context;
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::LitStr;
use syn::spanned::Spanned as _;

use super::super::Res;
use super::super::expression::{Identifier, ident, keyword};
use super::{Statement, StatementKind};
use crate::syntax::expression::Keyword;
use crate::syntax::template::whitespace;
use crate::{Source, State};

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
    ) -> Result<(TokenStream, usize), (TokenStream, usize)> {
        if state.default_escaper_group.is_some() {
            let span = self.escaper.span();
            let tag = self.tag.0.as_str();
            let tag_span = self.tag.span();
            let tag = quote_spanned! {tag_span=> #tag };
            Err((
                quote_spanned! {span=> compile_error!(concat!("Unexpected '", #tag, "' statement after already setting the default escaper group")); },
                0,
            ))
        } else if *state.has_content {
            let span = self.escaper.span();
            let tag = self.tag.0.as_str();
            let tag_span = self.tag.span();
            let tag = quote_spanned! {tag_span=> #tag };
            Err((
                quote_spanned! {span=> compile_error!(concat!("Unexpected '", #tag, "' statement after content already present in template")); },
                0,
            ))
        } else {
            if !self.can_replace_inferred_escaper
                && let Some(inferred_escaper_group) = &state.inferred_escaper_group
                && inferred_escaper_group.0 != self.escaper.ident
            {
                let default_escaper = LitStr::new(self.escaper.ident, self.escaper.span());
                let inferred_escaper_group = inferred_escaper_group.0;
                let span = self.escaper.source.span();
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
            if !state.config.escaper_groups.contains_key(self.escaper.ident) {
                let span = self.escaper.span();
                let default_escaper = LitStr::new(self.escaper.ident, span);
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

pub(super) fn parse_default_escaper_group(input: Source) -> Res<Source, Statement> {
    let (input, tag) = alt((
        keyword("default_escaper_group"),
        keyword("replace_escaper_group"),
    ))
    .parse(input)?;

    let (input, (leading_whitespace, escaper)) = cut((
        context("Expected space", whitespace),
        context("Expected an escaper group name", ident),
    ))
    .parse(input)?;

    let can_replace_inferred_escaper = match tag.0.as_str() {
        "default_escaper_group" => false,
        "replace_escaper_group" => true,
        _ => unreachable!("All tag cases should be covered"),
    };

    let source = tag
        .0
        .clone()
        .merge(&leading_whitespace, "Whitespace expected after tag name")
        .merge(&escaper.source, "Escaper name expected after whitespace");

    Ok((
        input,
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

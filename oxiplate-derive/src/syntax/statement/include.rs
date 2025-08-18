use std::collections::HashMap;
use std::fmt;

use nom::bytes::complete::{escaped, is_not, tag, take_while1};
use nom::character::complete::one_of;
use nom::combinator::cut;
use nom::error::context;
use nom::Parser as _;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::LitStr;

use super::super::expression::keyword;
use super::super::Res;
use super::{Statement, StatementKind};
use crate::syntax::template::is_whitespace;
use crate::{oxiplate_internal, Source};

pub struct Include<'a> {
    path: Source<'a>,
    template: TokenStream,
    estimated_length: usize,
}

impl fmt::Debug for Include<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Include")
            .field("path", &self.path)
            .field("template", &self.template)
            .finish()
    }
}

impl Include<'_> {
    /// Get the estimated length of all blocks.
    pub(crate) fn estimated_length(&self) -> usize {
        self.estimated_length
    }
}

impl<'a> From<Include<'a>> for StatementKind<'a> {
    fn from(statement: Include<'a>) -> Self {
        StatementKind::Include(statement)
    }
}

impl ToTokens for Include<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let template = &self.template;
        tokens.append_all(quote! { #template });
    }
}

pub(super) fn parse_include(input: Source) -> Res<Source, Statement> {
    let (input, include_keyword) = keyword("include").parse(input)?;

    let (input, (_, _, path, _)) = cut((
        context("Expected space after 'include'", take_while1(is_whitespace)),
        context(r#"Expected ""#, tag(r#"""#)),
        context(
            "Expected path to the template to include",
            escaped(is_not(r#"""#), '\\', one_of(r#"""#)),
        ),
        context(r#"Expected ""#, tag(r#"""#)),
    ))
    .parse(input)?;

    let source = include_keyword.0;
    let span = source.span();

    #[cfg(feature = "oxiplate")]
    let oxiplate = quote_spanned! {span=> ::oxiplate::Oxiplate };
    #[cfg(not(feature = "oxiplate"))]
    let oxiplate = quote_spanned! {span=> ::oxiplate_derive::Oxiplate };

    // Generate tokens for the included template.
    // They'll be injected into the main template later.
    //
    // `IncludingTemplate` doesn't include any fields
    // because the struct will be discarded
    // before type checks are done on the generated code.
    // If fields that don't exist are accessed,
    // Rust will handle the error message for those,
    // and the spans in the generated code
    // will point the user to the correct place in the code
    // to fix things.
    let include_path = LitStr::new(path.as_str(), path.span());
    let template = quote_spanned! {span=>
        #[derive(#oxiplate)]
        #[oxiplate_include = #include_path]
        struct IncludingTemplate;
    };
    let (template, estimated_length) = oxiplate_internal(template.into(), &HashMap::new());

    Ok((
        input,
        Statement {
            kind: Include {
                path: path.clone(),
                template: template.into(),
                estimated_length,
            }
            .into(),
            source,
        },
    ))
}

use std::collections::{HashMap, VecDeque};

use nom::Parser as _;
use nom::bytes::complete::{escaped, is_not, tag};
use nom::character::complete::one_of;
use nom::combinator::cut;
use nom::error::context;
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote, quote_spanned};
use syn::LitStr;

use super::super::Res;
use super::super::expression::keyword;
use super::{Statement, StatementKind};
use crate::syntax::template::whitespace;
use crate::{Source, Tokens, oxiplate_internal};

#[derive(Debug)]
pub struct Include<'a> {
    path: Source<'a>,
}

impl<'a> From<Include<'a>> for StatementKind<'a> {
    fn from(statement: Include<'a>) -> Self {
        StatementKind::Include(statement)
    }
}

impl Include<'_> {
    pub fn to_tokens(&self) -> Tokens {
        let mut tokens = TokenStream::new();

        let span = self.path.span();

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
        let include_path = LitStr::new(self.path.as_str(), self.path.span());
        let template = quote_spanned! {span=>
            #[derive(#oxiplate)]
            #[oxiplate_include = #include_path]
            struct IncludingTemplate;
        };
        let (template, estimated_length) =
            oxiplate_internal(template.into(), &VecDeque::from([&HashMap::new()]));
        let template: proc_macro2::TokenStream = template.into();

        tokens.append_all(quote! { #template });

        (tokens, estimated_length)
    }
}

pub(super) fn parse_include(input: Source) -> Res<Source, Statement> {
    let (input, include_keyword) = keyword("include").parse(input)?;

    let (input, (leading_whitespace, start_quote, path, end_quote)) = cut((
        context("Expected space after 'include'", whitespace),
        context(r#"Expected ""#, tag(r#"""#)),
        context(
            "Expected path to the template to include",
            escaped(is_not(r#"""#), '\\', one_of(r#"""#)),
        ),
        context(r#"Expected ""#, tag(r#"""#)),
    ))
    .parse(input)?;

    let source = include_keyword
        .0
        .merge(&leading_whitespace, "Whitespace expected after `include`")
        .merge(&start_quote, "`\"` expected after whitespace")
        .merge(&path, "Path expected after `\"`")
        .merge(&end_quote, "`\"` expected after path");

    Ok((
        input,
        Statement {
            kind: Include { path }.into(),
            source,
        },
    ))
}

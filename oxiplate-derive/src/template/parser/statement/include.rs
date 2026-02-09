use std::collections::{HashMap, VecDeque};

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, TokenStreamExt};
use syn::LitStr;

use super::{Statement, StatementKind};
use crate::parser::{cut, Parser as _};
use crate::template::parser::expression::{KeywordParser, String};
use crate::template::parser::Res;
use crate::template::tokenizer::TokenSlice;
use crate::{oxiplate_internal, BuiltTokens};

#[derive(Debug)]
pub struct Include<'a> {
    path: String<'a>,
}

impl<'a> From<Include<'a>> for StatementKind<'a> {
    fn from(statement: Include<'a>) -> Self {
        StatementKind::Include(statement)
    }
}

impl Include<'_> {
    pub fn to_tokens(&self) -> BuiltTokens {
        let mut tokens = TokenStream::new();

        let span = self.path.source().span_token();

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
        let include_path = LitStr::new(self.path.as_str(), self.path.source().span_token());
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

pub(super) fn parse_include(tokens: TokenSlice) -> Res<Statement> {
    let (tokens, (include_keyword, path)) = (
        KeywordParser::new("include"),
        cut("Expected path to the template to include", String::parse),
    )
        .parse(tokens)?;

    let source = include_keyword
        .source()
        .clone()
        .merge(path.source(), "Path expected after `include`");

    Ok((
        tokens,
        Statement {
            kind: Include { path }.into(),
            source,
        },
    ))
}

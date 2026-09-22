use std::collections::{HashMap, VecDeque};

use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote, quote_spanned};
use syn::{Ident, LitStr};

use super::{Statement, StatementKind};
use crate::parser::{Parser as _, cut};
use crate::template::parser::Res;
use crate::template::parser::expression::{KeywordParser, String};
use crate::template::tokenizer::TokenSlice;
use crate::{BuiltTokens, State, oxiplate_internal};

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
    pub fn to_tokens(&self, state: &State) -> BuiltTokens {
        let mut tokens = TokenStream::new();

        let span = self.path.source().span_token();

        #[cfg(feature = "_oxiplate")]
        let oxiplate = quote_spanned! {span=> ::oxiplate::Oxiplate };
        #[cfg(not(feature = "_oxiplate"))]
        let oxiplate = quote_spanned! {span=> ::oxiplate_derive::Oxiplate };

        // Generate tokens for the included template.
        // They'll be injected into the main template later.
        //
        // `IncludingTemplate` doesn't include types for any fields
        // because the struct will be discarded
        // before type checks are done on the generated code.
        // The fields names need to be included
        // so the state can be properly built
        // and `foo` in a template
        // can be turned into `self.foo`
        // in the generated Rust code.
        let include_path = LitStr::new(self.path.as_str(), self.path.source().span_token());
        let fields = state.fields.iter().map(|field| Ident::new_raw(field, span));
        let template = quote_spanned! {span=>
            #[derive(#oxiplate)]
            #[oxiplate_include = #include_path]
            struct IncludingTemplate {
                #(#fields: (),)*
            }
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

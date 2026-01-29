use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote_spanned};

use super::Res;
use crate::Source;
use crate::parser::Parser;
use crate::template::parser::Error;
use crate::template::parser::expression::Identifier;
use crate::template::tokenizer::{TokenKind, TokenSlice};

#[derive(Debug)]
pub(crate) struct Keyword<'a> {
    source: &'a Source<'a>,
}

impl<'a> Keyword<'a> {
    pub fn source(&self) -> &Source<'a> {
        self.source
    }
}

impl ToTokens for Keyword<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = self.source.span_token();
        let keyword = syn::Ident::new(self.source.as_str(), span);
        tokens.append_all(quote_spanned! {span=> #keyword });
    }
}

pub(crate) struct KeywordParser {
    keyword: &'static str,
}

impl KeywordParser {
    pub fn new(keyword: &'static str) -> Self {
        Self { keyword }
    }
}

impl<'a> Parser<'a, TokenKind> for KeywordParser {
    type Output = Keyword<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, ident) = Identifier::parse.parse(tokens)?;

        if ident.as_str() != self.keyword {
            return Err(Error::Recoverable {
                message: format!(
                    "Identifier `{}` did not match expected keyword `{}`",
                    ident.as_str(),
                    self.keyword
                ),
                source: ident.source().clone(),
                previous_error: None,
                is_eof: false,
            });
        }

        Ok((
            tokens,
            Keyword {
                source: ident.source(),
            },
        ))
    }
}

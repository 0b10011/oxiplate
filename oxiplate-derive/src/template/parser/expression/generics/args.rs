use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote_spanned};

use super::Generics;
use super::lifetime::Lifetime;
use crate::parser::{Parser as _, ignore_recoverable_errors, into, many0, take};
use crate::template::parser::Res;
use crate::template::tokenizer::{Token, TokenKind};
use crate::{Source, TokenSlice};

#[derive(Debug)]
pub(super) struct GenericArgs<'a> {
    first_generics: Vec<(GenericArg<'a>, Source<'a>)>,
    last_generic: GenericArg<'a>,
    trailing_comma: Option<Source<'a>>,
    source: Source<'a>,
}

impl<'a> GenericArgs<'a> {
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (less_than, (first_generics, last_generic, trailing_comma), greater_than)) = (
            take(TokenKind::LessThan),
            (
                many0((GenericArg::parse, take(TokenKind::Comma))),
                GenericArg::parse,
                ignore_recoverable_errors(take(TokenKind::Comma)),
            ),
            take(TokenKind::GreaterThan),
        )
            .parse(tokens)?;

        let mut source = less_than.source().clone();

        for (generic, comma) in &first_generics {
            source = source
                .merge(generic.source(), "Generic expected after `,` or `<`")
                .merge(comma.source(), "`,` expected after generic");
        }

        source = source
            .merge(last_generic.source(), "Generic expected after `,` or `<`")
            .merge_some(
                trailing_comma.map(Token::source),
                "`,` expected after last generic",
            )
            .merge(greater_than.source(), "`>` expected after generics");

        Ok((
            tokens,
            Self {
                first_generics: first_generics
                    .into_iter()
                    .map(|(generic, comma)| (generic, comma.source().clone()))
                    .collect(),
                last_generic,
                trailing_comma: trailing_comma.map(|comma| comma.source().clone()),
                source,
            },
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl ToTokens for GenericArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut inner_tokens = TokenStream::new();

        for (arg, comma) in &self.first_generics {
            let span = comma.span_token();
            inner_tokens.append_all(quote_spanned! {span=> #arg, });
        }

        self.last_generic.to_tokens(&mut inner_tokens);
        if let Some(comma) = &self.trailing_comma {
            let span = comma.span_token();
            inner_tokens.append_all(quote_spanned! {span=> , });
        }

        let span = self.source.span_token();
        tokens.append_all(quote_spanned! {span=> <#inner_tokens> });
    }
}

impl<'a> From<GenericArgs<'a>> for Generics<'a> {
    fn from(value: GenericArgs<'a>) -> Self {
        Self::GenericArgs(value)
    }
}

#[derive(Debug)]
pub(super) enum GenericArg<'a> {
    Lifetime(Lifetime<'a>),
}

impl<'a> GenericArg<'a> {
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        into(Lifetime::parse).parse(tokens)
    }

    pub(super) fn source(&self) -> &Source<'a> {
        match self {
            Self::Lifetime(lifetime) => lifetime.source(),
        }
    }
}

impl ToTokens for GenericArg<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Lifetime(lifetime) => lifetime.to_tokens(tokens),
        }
    }
}

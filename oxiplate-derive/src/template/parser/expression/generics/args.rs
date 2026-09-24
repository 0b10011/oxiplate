use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote_spanned};

use super::Generics;
use super::lifetime::Lifetime;
use crate::parser::{Parser as _, ignore_recoverable_errors, into, many0, take};
use crate::template::parser::Res;
use crate::template::tokenizer::TokenKind;
use crate::{Source, TokenSlice};

#[derive(Debug)]
pub(super) struct GenericArgs<'a> {
    first_generics: Vec<(GenericArg<'a>, Source<'a>)>,
    last_generic: Option<(GenericArg<'a>, Option<Source<'a>>)>,
    source: Source<'a>,
}

impl<'a> GenericArgs<'a> {
    pub(super) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (less_than, (mut first_generics, last_generic), greater_than)) = (
            take(TokenKind::LessThan),
            (
                many0((GenericArg::parse, take(TokenKind::Comma))),
                ignore_recoverable_errors((
                    GenericArg::parse,
                    ignore_recoverable_errors(take(TokenKind::Comma)),
                )),
            ),
            take(TokenKind::GreaterThan),
        )
            .parse(tokens)?;

        let last_generic = if last_generic.is_some() {
            last_generic
                .map(|(generic, comma)| (generic, comma.map(|comma| comma.source().clone())))
        } else {
            first_generics
                .pop()
                .map(|(generic, comma)| (generic, Some(comma.source().clone())))
        };

        let mut source = less_than.source().clone();

        for (generic, comma) in &first_generics {
            source = source
                .merge(generic.source(), "Generic expected after `,` or `<`")
                .merge(comma.source(), "`,` expected after generic");
        }

        if let Some((generic, comma)) = &last_generic {
            source = source
                .merge(generic.source(), "Generic expected after `,` or `<`")
                .merge_some(comma.as_ref(), "`,` expected after last generic");
        }

        source = source.merge(greater_than.source(), "`>` expected after generics");

        Ok((
            tokens,
            Self {
                first_generics: first_generics
                    .into_iter()
                    .map(|(generic, comma)| (generic, comma.source().clone()))
                    .collect(),
                last_generic,
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

        if let Some((generic, comma)) = &self.last_generic {
            generic.to_tokens(&mut inner_tokens);
            if let Some(comma) = comma {
                let span = comma.span_token();
                inner_tokens.append_all(quote_spanned! {span=> , });
            }
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

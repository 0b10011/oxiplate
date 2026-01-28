mod literal;
mod range;
mod r#struct;
mod tuple;

use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote, quote_spanned};

use self::literal::Literal;
use self::range::Range;
use self::r#struct::Struct;
use self::tuple::Tuple;
use crate::parser::{Parser as _, alt, cut, into, many1, opt, take};
use crate::syntax::Res;
use crate::syntax::expression::Identifier;
use crate::tokenizer::parser::{TokenKind, TokenSlice};
use crate::{Source, State};

#[derive(Debug)]
pub(crate) enum Pattern<'a> {
    Literal(Literal<'a>),
    Ident(Identifier<'a>),
    Range(Range<'a>),
    Struct(Struct<'a>),
    Tuple(Tuple<'a>),
    // TODO: foo @ 3..=7
}

impl<'a> Pattern<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((
            into(Range::parse),
            into(Literal::parse),
            into(Tuple::parse),
            into(Struct::parse),
            into(Identifier::parse),
        ))
        .parse(tokens)
    }

    pub fn source(&self) -> &Source<'a> {
        match self {
            Self::Literal(literal) => literal.source(),
            Self::Ident(identifier) => identifier.source(),
            Self::Range(range) => range.source(),
            Self::Struct(r#struct) => r#struct.source(),
            Self::Tuple(tuple) => tuple.source(),
        }
    }

    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        match self {
            Self::Ident(identifier) => HashSet::from([identifier.as_str()]),
            Self::Struct(value) => value.get_variables(),
            Self::Tuple(value) => value.get_variables(),
            Self::Literal(_) | Self::Range(_) => HashSet::new(),
        }
    }

    pub fn to_tokens(&self, state: &State) -> TokenStream {
        match self {
            Self::Literal(value) => value.to_tokens(),
            Self::Ident(value) => quote! { #value },
            Self::Range(value) => value.to_tokens(state),
            Self::Struct(value) => value.to_tokens(state),
            Self::Tuple(value) => value.to_tokens(state),
        }
    }
}

impl<'a> From<Identifier<'a>> for Pattern<'a> {
    fn from(value: Identifier<'a>) -> Self {
        Pattern::Ident(value)
    }
}

#[derive(Debug)]
pub(crate) struct Path<'a> {
    segments: Vec<(Identifier<'a>, Source<'a>)>,
    name: Identifier<'a>,
    source: Source<'a>,
}

impl<'a> Path<'a> {
    pub fn parse_include_ident(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, path) = opt(Self::parse_exclude_ident).parse(tokens)?;

        if let Some(path) = path {
            Ok((tokens, path))
        } else {
            let (tokens, name) = Identifier::parse.parse(tokens)?;
            let source = name.source().clone();
            Ok((
                tokens,
                Self {
                    segments: vec![],
                    name,
                    source,
                },
            ))
        }
    }

    pub fn parse_exclude_ident(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (segments_with_whitespace, name)) = (
            many1((Identifier::parse, take(TokenKind::PathSeparator))),
            cut("Expected type name and optional fields", Identifier::parse),
        )
            .parse(tokens)?;

        let mut segments = vec![];
        let mut source: Option<Source<'a>> = None;
        for (segment, colons) in segments_with_whitespace {
            if let Some(existing_source) = source {
                source = Some(
                    existing_source
                        .merge(segment.source(), "Segment expected")
                        .merge(colons.source(), "Colons expected after whitespace"),
                );
            } else {
                source = Some(
                    segment
                        .source()
                        .clone()
                        .merge(colons.source(), "Colons expected after whitespace"),
                );
            }

            segments.push((segment, colons.source().clone()));
        }

        let source = source
            .expect("Source should already exist because at least one segment is required")
            .merge(name.source(), "Name expected after segments");

        Ok((
            tokens,
            Self {
                segments,
                name,
                source,
            },
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn to_tokens(&self) -> TokenStream {
        let mut tokens = TokenStream::new();

        for (segment, colons) in &self.segments {
            let colons_span = colons.span_token();
            let colons = quote_spanned! {colons_span=> :: };
            tokens.append_all(quote! { #segment #colons });
        }

        self.name.to_tokens(&mut tokens);

        tokens
    }
}

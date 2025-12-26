use std::collections::HashSet;

use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, into, opt};
use nom::error::context;
use nom::multi::many1;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote, quote_spanned};

mod literal;
mod range;
mod r#struct;
mod tuple;

use self::literal::Literal;
use self::range::Range;
use self::r#struct::Struct;
use self::tuple::Tuple;
use crate::syntax::Res;
use crate::syntax::expression::Identifier;
use crate::syntax::template::whitespace;
use crate::{Source, State};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Pattern<'a> {
    Literal(Literal<'a>),
    Ident(Identifier<'a>),
    Range(Range<'a>),
    Struct(Struct<'a>),
    Tuple(Tuple<'a>),
    // TODO: foo @ 3..=7
}

impl<'a> Pattern<'a> {
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        alt((
            into(Range::parse),
            into(Literal::parse),
            into(Tuple::parse),
            into(Struct::parse),
            into(Identifier::parse),
        ))
        .parse(input)
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

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Path<'a> {
    segments: Vec<(Identifier<'a>, Source<'a>)>,
    name: Identifier<'a>,
    source: Source<'a>,
}

impl<'a> Path<'a> {
    pub fn parse_include_ident(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, path) = opt(Self::parse_exclude_ident).parse(input)?;

        if let Some(path) = path {
            Ok((input, path))
        } else {
            let (input, name) = Identifier::parse.parse(input)?;
            let source = name.source().clone();
            Ok((
                input,
                Self {
                    segments: vec![],
                    name,
                    source,
                },
            ))
        }
    }

    pub fn parse_exclude_ident(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, (segments_with_whitespace, name)) = (
            many1((
                Identifier::parse,
                opt(whitespace),
                tag("::"),
                opt(whitespace),
            )),
            context(
                "Expected type name and optional fields",
                cut(Identifier::parse),
            ),
        )
            .parse(input)?;

        let mut segments = vec![];
        let mut source: Option<Source<'a>> = None;
        for (segment, leading_whitespace, colons, trailing_whitespace) in segments_with_whitespace {
            if let Some(existing_source) = source {
                source = Some(
                    existing_source
                        .merge(segment.source(), "Segment expected")
                        .merge_some(
                            leading_whitespace.as_ref(),
                            "Whitespace expected after segment",
                        )
                        .merge(&colons, "Colons expected after whitespace")
                        .merge_some(
                            trailing_whitespace.as_ref(),
                            "Whitespace expected after colons",
                        ),
                );
            } else {
                source = Some(
                    segment
                        .source()
                        .clone()
                        .merge_some(
                            leading_whitespace.as_ref(),
                            "Whitespace expected after segment",
                        )
                        .merge(&colons, "Colons expected after whitespace")
                        .merge_some(
                            trailing_whitespace.as_ref(),
                            "Whitespace expected after colons",
                        ),
                );
            }

            segments.push((segment, colons));
        }

        let source = source
            .expect("Source should already exist because at least one segment is required")
            .merge(name.source(), "Name expected after segments");

        Ok((
            input,
            Self {
                segments,
                name,
                source,
            },
        ))
    }

    pub fn source<'b>(&'b self) -> &'b Source<'a> {
        &self.source
    }

    pub fn to_tokens(&self) -> TokenStream {
        let mut tokens = TokenStream::new();

        for (segment, colons) in &self.segments {
            let colons_span = colons.span();
            let colons = quote_spanned! {colons_span=> :: };
            tokens.append_all(quote! { #segment #colons });
        }

        self.name.to_tokens(&mut tokens);

        tokens
    }
}

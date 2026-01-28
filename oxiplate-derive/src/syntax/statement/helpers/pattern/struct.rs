use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote, quote_spanned};

use super::Pattern;
use crate::syntax::Res;
use crate::syntax::expression::Identifier;
use crate::syntax::parser::{Parser as _, alt, context, cut, into, many0, take};
use crate::syntax::statement::helpers::pattern::Path;
use crate::tokenizer::parser::{TokenKind, TokenSlice};
use crate::{BuiltTokens, Source, State};

#[derive(Debug)]
pub(crate) enum Struct<'a> {
    Named(NamedStruct<'a>),
    Tuple(TupleStruct<'a>),
    Unit(Path<'a>),
}

impl<'a> Struct<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((
            into(NamedStruct::parse),
            into(TupleStruct::parse),
            into(Path::parse_exclude_ident),
        ))
        .parse(tokens)
    }

    pub fn source(&self) -> &Source<'a> {
        match self {
            Self::Named(named_struct) => named_struct.source(),
            Self::Tuple(tuple_struct) => tuple_struct.source(),
            Self::Unit(path) => path.source(),
        }
    }

    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        match self {
            Self::Named(named_struct) => named_struct.get_variables(),
            Self::Tuple(tuple_struct) => tuple_struct.get_variables(),
            Self::Unit(_path) => HashSet::new(),
        }
    }

    pub fn to_tokens(&self, state: &State) -> TokenStream {
        match self {
            Self::Named(named_struct) => named_struct.to_tokens(state),
            Self::Tuple(tuple_struct) => tuple_struct.to_tokens(state),
            Self::Unit(path) => path.to_tokens(),
        }
    }
}

impl<'a> From<Struct<'a>> for Pattern<'a> {
    fn from(value: Struct<'a>) -> Self {
        Self::Struct(value)
    }
}

impl<'a> From<Path<'a>> for Struct<'a> {
    fn from(value: Path<'a>) -> Self {
        Self::Unit(value)
    }
}

#[derive(Debug)]
pub(crate) struct NamedStruct<'a> {
    path: Path<'a>,
    first_field: Field<'a>,
    /// `Source` is the comma.
    additional_fields: Vec<(Source<'a>, Field<'a>)>,
    /// `..`
    #[allow(dead_code)]
    remaining: Option<Source<'a>>,
    source: Source<'a>,
}

impl<'a> NamedStruct<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (path, open_brace, (first_field, additional_fields_and_commas, close_brace))) =
            (
                Path::parse_include_ident,
                take(TokenKind::OpenBrace),
                (
                    cut("Expected a field", Field::parse),
                    many0((
                        cut("Expected `,`", take(TokenKind::Comma)),
                        cut("Expected a field", Field::parse),
                    )),
                    cut("Expected `}`", take(TokenKind::CloseBrace)),
                ),
            )
                .parse(tokens)?;

        let mut source = path
            .source
            .clone()
            .merge(open_brace.source(), "Open brace expected after path")
            .merge(
                first_field.source(),
                "First field expected after whitespace",
            );

        let mut additional_fields = Vec::with_capacity(additional_fields_and_commas.len());
        for (comma, field) in additional_fields_and_commas {
            source = source
                .merge(
                    comma.source(),
                    "Comma and whitespace expected after whitespace",
                )
                .merge(field.source(), "Field expected after whitespace");

            additional_fields.push((comma.source().clone(), field));
        }

        source = source.merge(
            close_brace.source(),
            "Closing brace expected after whitespace",
        );

        Ok((
            tokens,
            Self {
                path,
                first_field,
                additional_fields,
                remaining: None,
                source,
            },
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        let mut vars: HashSet<&'a str> = self.first_field.get_variables();

        for (_comma, field) in &self.additional_fields {
            vars.extend(field.get_variables());
        }

        vars
    }

    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let path = self.path.to_tokens();

        let (mut tokens, _expected_length) = self.first_field.to_tokens(state);

        for (comma, value) in &self.additional_fields {
            let comma_span = comma.span_token();
            let comma = quote_spanned! {comma_span=> , };
            let (value, _expected_length) = value.to_tokens(state);
            tokens.append_all([comma, value]);
        }

        let span = self.source.span_token();

        quote_spanned! {span=> #path { #tokens } }
    }
}

impl<'a> From<NamedStruct<'a>> for Struct<'a> {
    fn from(value: NamedStruct<'a>) -> Self {
        Struct::Named(value)
    }
}

#[derive(Debug)]
struct Field<'a> {
    name: Identifier<'a>,
    value: Option<Box<Pattern<'a>>>,
    source: Source<'a>,
}

impl<'a> Field<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((Self::parse_with_value, Self::parse_without_value)).parse(tokens)
    }

    pub fn parse_with_value(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (name, colon, pattern)) = (
            context("Expected field name", Identifier::parse),
            context("Expected `:`", take(TokenKind::Colon)),
            cut("Expected pattern", Pattern::parse),
        )
            .parse(tokens)?;

        let source = name
            .source()
            .clone()
            .merge(colon.source(), "Colon expected after name")
            .merge(pattern.source(), "Pattern expected after colon");

        Ok((
            tokens,
            Self {
                name,
                value: Some(Box::new(pattern)),
                source,
            },
        ))
    }

    pub fn parse_without_value(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, name) = context("Expected field name", Identifier::parse).parse(tokens)?;

        Ok((
            tokens,
            Self {
                source: name.source().clone(),
                name,
                value: None,
            },
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        let Some(value) = &self.value else {
            return HashSet::from([self.name.as_str()]);
        };

        value.get_variables()
    }

    pub fn to_tokens(&self, state: &State) -> BuiltTokens {
        let name = &self.name;

        if let Some(value) = &self.value {
            let span = self.source.span_token();
            let value = value.to_tokens(state);
            (quote_spanned! {span=> #name: #value }, 0)
        } else {
            (quote! { #name }, 0)
        }
    }
}

#[derive(Debug)]
pub(crate) struct TupleStruct<'a> {
    path: Path<'a>,
    first_field: Box<Pattern<'a>>,
    /// `Source` is the comma.
    additional_fields: Vec<(Source<'a>, Pattern<'a>)>,
    /// `..`
    #[allow(dead_code)]
    remaining: Option<Source<'a>>,
    source: Source<'a>,
}

impl<'a> TupleStruct<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (path, open_paren, (first_field, additional_fields_and_commas, close_brace))) =
            (
                Path::parse_include_ident,
                take(TokenKind::OpenParenthese),
                (
                    cut("Expected pattern", Pattern::parse),
                    many0((
                        cut("Expected `,`", take(TokenKind::Comma)),
                        cut("Expected a pattern", Pattern::parse),
                    )),
                    cut("Expected `)`", take(TokenKind::CloseParenthese)),
                ),
            )
                .parse(tokens)?;

        let mut source = path
            .source
            .clone()
            .merge(open_paren.source(), "Open parenthese expected after path")
            .merge(
                first_field.source(),
                "First field expected after open parenthese",
            );

        let mut additional_fields = Vec::with_capacity(additional_fields_and_commas.len());
        for (comma, field) in additional_fields_and_commas {
            source = source
                .merge(comma.source(), "Comma expected after previous field")
                .merge(field.source(), "Field expected after comma");

            additional_fields.push((comma.source().clone(), field));
        }

        source = source.merge(
            close_brace.source(),
            "Closing brace expected after previous field",
        );

        Ok((
            tokens,
            Self {
                path,
                first_field: Box::new(first_field),
                additional_fields,
                remaining: None,
                source,
            },
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        let mut vars: HashSet<&'a str> = self.first_field.get_variables();

        for (_comma, field) in &self.additional_fields {
            vars.extend(field.get_variables());
        }

        vars
    }

    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let path = self.path.to_tokens();

        let mut tokens = self.first_field.to_tokens(state);

        for (comma, value) in &self.additional_fields {
            let comma_span = comma.span_token();
            let comma = quote_spanned! {comma_span=> , };
            let value = value.to_tokens(state);
            tokens.append_all([comma, value]);
        }

        let span = self.source.span_token();

        quote_spanned! {span=> #path(#tokens) }
    }
}

impl<'a> From<TupleStruct<'a>> for Struct<'a> {
    fn from(value: TupleStruct<'a>) -> Self {
        Struct::Tuple(value)
    }
}

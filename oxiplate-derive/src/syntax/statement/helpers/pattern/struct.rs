use std::collections::HashSet;

use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, into, opt};
use nom::error::context;
use nom::multi::many0;
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote, quote_spanned};

use super::Pattern;
use crate::syntax::Res;
use crate::syntax::expression::Identifier;
use crate::syntax::statement::helpers::pattern::Path;
use crate::syntax::template::whitespace;
use crate::{BuiltTokens, Source, State};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Struct<'a> {
    Named(NamedStruct<'a>),
    Tuple(TupleStruct<'a>),
    Unit(Path<'a>),
}

impl<'a> Struct<'a> {
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        alt((
            into(NamedStruct::parse),
            into(TupleStruct::parse),
            into(Path::parse_exclude_ident),
        ))
        .parse(input)
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

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NamedStruct<'a> {
    path: Path<'a>,
    first_field: Field<'a>,
    /// `Source` is the comma.
    additional_fields: Vec<(Source<'a>, Field<'a>)>,
    /// `..`
    remaining: Option<Source<'a>>,
    source: Source<'a>,
}

impl<'a> NamedStruct<'a> {
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (
            input,
            (
                path,
                leading_whitespace,
                open_brace,
                middle_whitespace,
                (first_field, additional_fields_and_whitespace, trailing_whitespace, close_brace),
            ),
        ) = (
            Path::parse_include_ident,
            opt(whitespace),
            tag("{"),
            opt(whitespace),
            cut((
                Field::parse,
                many0((opt(whitespace), tag(","), opt(whitespace), Field::parse)),
                opt(whitespace),
                tag("}"),
            )),
        )
            .parse(input)?;

        let mut source = path
            .source
            .clone()
            .merge_some(
                leading_whitespace.as_ref(),
                "Whitespace expected after path",
            )
            .merge(&open_brace, "Open brace expected after whitespace")
            .merge_some(middle_whitespace.as_ref(), "Whitespace expected after `{`")
            .merge(
                first_field.source(),
                "First field expected after whitespace",
            );

        let mut additional_fields = Vec::with_capacity(additional_fields_and_whitespace.len());
        for (leading_whitespace, comma, trailing_whitespace, field) in
            additional_fields_and_whitespace
        {
            let comma_source = if let Some(source) = leading_whitespace {
                source.merge(&comma, "Comma expected after whitespace")
            } else {
                comma.clone()
            }
            .merge_some(
                trailing_whitespace.as_ref(),
                "Whitespace expected after comma",
            );

            source = source
                .merge(
                    &comma_source,
                    "Comma and whitespace expected after whitespace",
                )
                .merge(field.source(), "Field expected after whitespace");

            additional_fields.push((comma_source, field));
        }

        source = source
            .merge_some(
                trailing_whitespace.as_ref(),
                "Whitespace expected after last field",
            )
            .merge(&close_brace, "Closing brace expected after whitespace");

        Ok((
            input,
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
            let comma_span = comma.span();
            let comma = quote_spanned! {comma_span=> , };
            let (value, _expected_length) = value.to_tokens(state);
            tokens.append_all([comma, value]);
        }

        let span = self.source.span();

        quote_spanned! {span=> #path { #tokens } }
    }
}

impl<'a> From<NamedStruct<'a>> for Struct<'a> {
    fn from(value: NamedStruct<'a>) -> Self {
        Struct::Named(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Field<'a> {
    name: Identifier<'a>,
    value: Option<Box<Pattern<'a>>>,
    source: Source<'a>,
}

impl<'a> Field<'a> {
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, (name, value)) = (
            context("Expected field name", Identifier::parse),
            opt((
                opt(whitespace),
                context("Expected `:`", tag(":")),
                opt(whitespace),
                context("Expected type or ident", cut(Pattern::parse)),
            )),
        )
            .parse(input)?;

        let field = if let Some((leading_whitespace, colon, trailing_whitespace, ty)) = value {
            let source = name
                .source()
                .clone()
                .merge_some(
                    leading_whitespace.as_ref(),
                    "Whitespace expected after name",
                )
                .merge(&colon, "Colon expected after whitespace")
                .merge_some(
                    trailing_whitespace.as_ref(),
                    "Whitespace expected after colon",
                )
                .merge(ty.source(), "Type expected after whitespace");

            Self {
                name,
                value: Some(Box::new(ty)),
                source,
            }
        } else {
            let source = name.source().clone();
            Self {
                name,
                value: None,
                source,
            }
        };

        Ok((input, field))
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
            let span = self.source.span();
            let value = value.to_tokens(state);
            (quote_spanned! {span=> #name: #value }, 0)
        } else {
            (quote! { #name }, 0)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TupleStruct<'a> {
    path: Path<'a>,
    first_field: Box<Pattern<'a>>,
    /// `Source` is the comma.
    additional_fields: Vec<(Source<'a>, Pattern<'a>)>,
    /// `..`
    remaining: Option<Source<'a>>,
    source: Source<'a>,
}

impl<'a> TupleStruct<'a> {
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (
            input,
            (
                path,
                leading_whitespace,
                open_brace,
                middle_whitespace,
                (first_field, additional_fields_and_whitespace, trailing_whitespace, close_brace),
            ),
        ) = (
            Path::parse_include_ident,
            opt(whitespace),
            tag("("),
            opt(whitespace),
            cut((
                context("Expected pattern", Pattern::parse),
                many0((opt(whitespace), tag(","), opt(whitespace), Pattern::parse)),
                opt(whitespace),
                context("Expected `)`", tag(")")),
            )),
        )
            .parse(input)?;

        let mut source = path
            .source
            .clone()
            .merge_some(
                leading_whitespace.as_ref(),
                "Whitespace expected after path",
            )
            .merge(&open_brace, "Open brace expected after whitespace")
            .merge_some(middle_whitespace.as_ref(), "Whitespace expected after `{`")
            .merge(
                first_field.source(),
                "First field expected after whitespace",
            );

        let mut additional_fields = Vec::with_capacity(additional_fields_and_whitespace.len());
        for (leading_whitespace, comma, trailing_whitespace, field) in
            additional_fields_and_whitespace
        {
            let comma_source = if let Some(source) = leading_whitespace {
                source.merge(&comma, "Comma expected after whitespace")
            } else {
                comma.clone()
            }
            .merge_some(
                trailing_whitespace.as_ref(),
                "Whitespace expected after comma",
            );

            source = source
                .merge(
                    &comma_source,
                    "Comma and whitespace expected after whitespace",
                )
                .merge(field.source(), "Field expected after whitespace");

            additional_fields.push((comma_source, field));
        }

        source = source
            .merge_some(
                trailing_whitespace.as_ref(),
                "Whitespace expected after last field",
            )
            .merge(&close_brace, "Closing brace expected after whitespace");

        Ok((
            input,
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
            let comma_span = comma.span();
            let comma = quote_spanned! {comma_span=> , };
            let value = value.to_tokens(state);
            tokens.append_all([comma, value]);
        }

        let span = self.source.span();

        quote_spanned! {span=> #path(#tokens) }
    }
}

impl<'a> From<TupleStruct<'a>> for Struct<'a> {
    fn from(value: TupleStruct<'a>) -> Self {
        Struct::Tuple(value)
    }
}

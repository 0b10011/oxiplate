use std::collections::HashSet;

use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::{cut, opt};
use nom::multi::many0;
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote_spanned};

use super::Pattern;
use crate::syntax::Res;
use crate::syntax::template::whitespace;
use crate::{Source, State};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Tuple<'a> {
    first_value: Box<Pattern<'a>>,
    /// `Source` is the comma.
    additional_values: Vec<(Source<'a>, Pattern<'a>)>,
    /// `..`
    remaining: Option<Source<'a>>,
    source: Source<'a>,
}

impl<'a> Tuple<'a> {
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (
            input,
            (
                leading_whitespace,
                open_brace,
                middle_whitespace,
                (first_field, additional_fields_and_whitespace, trailing_whitespace, close_brace),
            ),
        ) = (
            opt(whitespace),
            tag("("),
            opt(whitespace),
            cut((
                Pattern::parse,
                many0((opt(whitespace), tag(","), opt(whitespace), Pattern::parse)),
                opt(whitespace),
                tag(")"),
            )),
        )
            .parse(input)?;

        let mut source = if let Some(leading_whitespace) = leading_whitespace {
            leading_whitespace.merge(&open_brace, "Open brace expected after whitespace")
        } else {
            open_brace
        }
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
                first_value: Box::new(first_field),
                additional_values: additional_fields,
                remaining: None,
                source,
            },
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        let mut vars = self.first_value.get_variables();

        for (_comma, value) in &self.additional_values {
            vars.extend(value.get_variables());
        }

        vars
    }

    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let mut tokens = self.first_value.to_tokens(state);

        for (comma, value) in &self.additional_values {
            let comma_span = comma.span();
            let comma = quote_spanned! {comma_span=> , };
            let value = value.to_tokens(state);
            tokens.append_all([comma, value]);
        }

        let span = self.source.span();

        quote_spanned! {span=> (#tokens) }
    }
}

impl<'a> From<Tuple<'a>> for Pattern<'a> {
    fn from(value: Tuple<'a>) -> Self {
        Pattern::Tuple(value)
    }
}

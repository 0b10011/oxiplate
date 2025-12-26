use std::collections::HashSet;

use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::{cut, opt};
use nom::multi::many1;
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote_spanned};

use super::Pattern;
use crate::syntax::Res;
use crate::syntax::template::whitespace;
use crate::{Source, State};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Tuple<'a> {
    /// `Source` is the trailing comma.
    values: Vec<(Pattern<'a>, Source<'a>)>,
    last_value: Option<Box<Pattern<'a>>>,
    /// `..`
    remaining: Option<Source<'a>>,
    source: Source<'a>,
}

impl<'a> Tuple<'a> {
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (
            input,
            (open_brace, middle_whitespace, (fields_and_whitespace, last_field, close_brace)),
        ) = (
            tag("("),
            opt(whitespace),
            cut((
                many1((Pattern::parse, opt(whitespace), tag(","), opt(whitespace))),
                opt((
                    Pattern::parse,
                    opt(whitespace),
                    opt(tag(",")),
                    opt(whitespace),
                )),
                tag(")"),
            )),
        )
            .parse(input)?;

        let mut source =
            open_brace.merge_some(middle_whitespace.as_ref(), "Whitespace expected after `{`");

        let mut values = Vec::with_capacity(fields_and_whitespace.len());
        for (field, middle_whitespace, comma, trailing_whitespace) in fields_and_whitespace {
            source = source
                .merge(field.source(), "Field expected")
                .merge_some(
                    middle_whitespace.as_ref(),
                    "Whitespace expected after value",
                )
                .merge(&comma, "Comma expected after whitespace")
                .merge_some(
                    trailing_whitespace.as_ref(),
                    "Whitespace expected after comma",
                );

            values.push((field, comma));
        }

        let last_value =
            if let Some((last_field, middle_whitespace, comma, trailing_whitespace)) = last_field {
                source = source
                    .merge(last_field.source(), "Field expected after whitespace")
                    .merge_some(
                        middle_whitespace.as_ref(),
                        "Whitespace expected after field",
                    )
                    .merge_some(comma.as_ref(), "Comma expected after whitespace")
                    .merge_some(
                        trailing_whitespace.as_ref(),
                        "Whitespace expected after comma",
                    );
                Some(Box::new(last_field))
            } else {
                None
            };

        source = source.merge(&close_brace, "Closing brace expected after whitespace");

        Ok((
            input,
            Self {
                values,
                last_value,
                remaining: None,
                source,
            },
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        let mut vars = HashSet::new();

        for (value, _comma) in &self.values {
            vars.extend(value.get_variables());
        }

        if let Some(last_value) = &self.last_value {
            vars.extend(last_value.get_variables());
        }

        vars
    }

    pub fn to_tokens(&self, state: &State) -> TokenStream {
        let mut tokens = TokenStream::new();

        for (value, comma) in &self.values {
            let comma_span = comma.span();
            let comma = quote_spanned! {comma_span=> , };
            let value = value.to_tokens(state);
            tokens.append_all([value, comma]);
        }

        if let Some(last_value) = &self.last_value {
            tokens.append_all(last_value.to_tokens(state));
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

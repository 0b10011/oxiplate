use nom::Parser as _;
use nom::bytes::complete::{tag, take, take_while};
use nom::combinator::cut;
use nom::error::context;
use nom::multi::many_till;
use nom::sequence::pair;
use proc_macro2::TokenStream;
use quote::quote;

use crate::Source;
use crate::syntax::expression::{Expression, Res};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct String<'a> {
    value: Source<'a>,
    source: Source<'a>,
}

impl<'a> String<'a> {
    pub(crate) fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, (opening_hashes, opening_quote)) =
            pair(take_while(|c| c == '#'), tag("\"")).parse(input)?;

        let closing = pair(tag("\""), tag(opening_hashes.as_str()));
        let (input, (string, (closing_quote, closing_hashes))) = context(
        r#"String is opened but never closed. The string ending must be a double quote (") followed by the same number of hashes (#) as the string opening."#,
        cut(many_till(take(1u32), closing)),
    ).parse(input)?;

        let value = if let Some(value) = string.first() {
            let mut value = value.clone();
            value.range.end = string.last().unwrap().range.end;
            value
        } else {
            let mut value = opening_quote.clone();
            value.range.start = value.range.end;
            value
        };

        let source = opening_hashes
            .merge(&opening_quote, "Opening quote should follow opening hashes")
            .merge(&value, "Value should follow opening quote")
            .merge(&closing_quote, "Closing quote should follow value")
            .merge(
                &closing_hashes,
                "Closing hashes should follow closing quote",
            );

        Ok((input, Self { value, source }))
    }

    pub(crate) fn as_str(&self) -> &'a str {
        self.value.as_str()
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub(crate) fn to_tokens(&self) -> (TokenStream, usize) {
        let literal = ::syn::LitStr::new(self.value.as_str(), self.source.span());
        (quote! { #literal }, self.value.as_str().len())
    }
}

impl<'a> From<String<'a>> for Expression<'a> {
    fn from(value: String<'a>) -> Self {
        Expression::String(value)
    }
}

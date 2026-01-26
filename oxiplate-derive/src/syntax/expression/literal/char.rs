use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::none_of;
use nom::combinator::{cut, peek};
use nom::error::context;
use nom::sequence::preceded;
use quote::quote;

use crate::syntax::expression::{Expression, Res};
use crate::{BuiltTokens, Source, internal_error};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Char<'a> {
    value: char,
    source: Source<'a>,
}

impl<'a> Char<'a> {
    /// Parse char literal (e.g., `'a'`).
    /// See: <https://doc.rust-lang.org/reference/tokens.html#character-literals>
    pub(crate) fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, (opening_quote, value, closing_quote)) = (
            tag("'"),
            context(
                r"Expected `\'`, `\\`, or a single char followed by `'`.",
                cut(alt((
                    // Char
                    preceded(peek(none_of("'\\\n\r\t")), take(1usize)),
                    // Quote/ascii escape
                    alt((
                        tag(r"\'"),
                        tag(r#"\""#),
                        tag(r"\n"),
                        tag(r"\r"),
                        tag(r"\t"),
                        tag(r"\\"),
                        tag(r"\0"),
                        #[cfg(feature = "unreachable")]
                        tag(r"'1"),
                        #[cfg(feature = "unreachable")]
                        tag(r""),
                    )),
                ))),
            ),
            context(r"Expected `'`.", cut(tag("'"))),
        )
            .parse(input)?;

        let source = opening_quote
            .merge(&value, "Char should follow opening quote")
            .merge(&closing_quote, "Closing quote should follow char");

        let value = match value.as_str() {
            r"\'" => '\'',
            r#"\""# => '"',
            r"\n" => '\n',
            r"\r" => '\r',
            r"\t" => '\t',
            r"\\" => '\\',
            r"\0" => '\0',
            str => {
                let mut chars = str.chars();
                let Some(char) = chars.next() else {
                    internal_error!(source.span().unwrap(), "No char present in char expression");
                };
                if chars.count() > 0 {
                    internal_error!(
                        source.span().unwrap(),
                        "More than one char present in char expression",
                    );
                }
                char
            }
        };

        Ok((input, Self { value, source }))
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub(crate) fn to_tokens(&self) -> BuiltTokens {
        let literal = ::syn::LitChar::new(self.value, self.source.span());
        (quote! { #literal }, 1)
    }
}

impl<'a> From<Char<'a>> for Expression<'a> {
    fn from(value: Char<'a>) -> Self {
        Expression::Char(value)
    }
}

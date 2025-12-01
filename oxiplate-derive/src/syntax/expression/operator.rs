use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::tag;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote_spanned};

use super::super::Res;
use crate::Source;

pub(super) fn parse_operator(input: Source) -> Res<Source, Operator> {
    let (input, operator) = alt((
        tag("+"),
        tag("-"),
        tag("*"),
        tag("/"),
        tag("%"),
        tag("=="),
        tag("!="),
        tag(">="),
        tag("<="),
        tag(">"),
        tag("<"),
        tag("||"),
        tag("&&"),
        tag("..="),
        tag(".."),
    ))
    .parse(input)?;

    let operator = match operator.as_str() {
        "+" => Operator::Addition(operator),
        "-" => Operator::Subtraction(operator),
        "*" => Operator::Multiplication(operator),
        "/" => Operator::Division(operator),
        "%" => Operator::Remainder(operator),

        "==" => Operator::Equal(operator),
        "!=" => Operator::NotEqual(operator),
        ">" => Operator::GreaterThan(operator),
        "<" => Operator::LessThan(operator),
        ">=" => Operator::GreaterThanOrEqual(operator),
        "<=" => Operator::LessThanOrEqual(operator),

        "||" => Operator::Or(operator),
        "&&" => Operator::And(operator),

        "..=" => Operator::RangeInclusive(operator),
        ".." => Operator::RangeExclusive(operator),

        // coverage:ignore
        _ => unreachable!("All cases should be covered"),
    };

    Ok((input, operator))
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Operator<'a> {
    Addition(Source<'a>),
    Subtraction(Source<'a>),
    Multiplication(Source<'a>),
    Division(Source<'a>),
    Remainder(Source<'a>),

    Equal(Source<'a>),
    NotEqual(Source<'a>),
    GreaterThan(Source<'a>),
    LessThan(Source<'a>),
    GreaterThanOrEqual(Source<'a>),
    LessThanOrEqual(Source<'a>),

    Or(Source<'a>),
    And(Source<'a>),

    /// `start..=end` that matches all values where `start <= x <= end`.
    /// See: <https://doc.rust-lang.org/core/ops/struct.RangeInclusive.html>
    RangeInclusive(Source<'a>),

    /// `start..end` that matches all values where `start <= x < end`.
    /// `start..` that matches all values where `start <= x`.
    /// See: <https://doc.rust-lang.org/core/ops/struct.Range.html>
    RangeExclusive(Source<'a>),
}

impl<'a> Operator<'a> {
    pub(super) fn requires_expression_after(&self) -> bool {
        match self {
            Operator::Addition(_)
            | Operator::Subtraction(_)
            | Operator::Multiplication(_)
            | Operator::Division(_)
            | Operator::Remainder(_)
            | Operator::Equal(_)
            | Operator::NotEqual(_)
            | Operator::GreaterThan(_)
            | Operator::LessThan(_)
            | Operator::GreaterThanOrEqual(_)
            | Operator::LessThanOrEqual(_)
            | Operator::Or(_)
            | Operator::And(_)
            | Operator::RangeInclusive(_) => true,

            // `expr..` is valid as well as `expr..expr`.
            Operator::RangeExclusive(_) => false,
        }
    }

    /// Get the `Source` for the operator.
    pub fn source<'b>(&'b self) -> &'b Source<'a> {
        match self {
            Operator::Addition(source)
            | Operator::Subtraction(source)
            | Operator::Multiplication(source)
            | Operator::Division(source)
            | Operator::Remainder(source)
            | Operator::Equal(source)
            | Operator::NotEqual(source)
            | Operator::GreaterThan(source)
            | Operator::LessThan(source)
            | Operator::GreaterThanOrEqual(source)
            | Operator::LessThanOrEqual(source)
            | Operator::Or(source)
            | Operator::And(source)
            | Operator::RangeInclusive(source)
            | Operator::RangeExclusive(source) => source,
        }
    }
}

impl ToTokens for Operator<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Operator::Addition(source) => {
                let span = source.span();
                quote_spanned!(span=> +)
            }
            Operator::Subtraction(source) => {
                let span = source.span();
                quote_spanned!(span=> -)
            }
            Operator::Multiplication(source) => {
                let span = source.span();
                quote_spanned!(span=> *)
            }
            Operator::Division(source) => {
                let span = source.span();
                quote_spanned!(span=> /)
            }
            Operator::Remainder(source) => {
                let span = source.span();
                quote_spanned!(span=> %)
            }

            Operator::Equal(source) => {
                let span = source.span();
                quote_spanned!(span=> ==)
            }
            Operator::NotEqual(source) => {
                let span = source.span();
                quote_spanned!(span=> !=)
            }
            Operator::GreaterThan(source) => {
                let span = source.span();
                quote_spanned!(span=> >)
            }
            Operator::LessThan(source) => {
                let span = source.span();
                quote_spanned!(span=> <)
            }
            Operator::GreaterThanOrEqual(source) => {
                let span = source.span();
                quote_spanned!(span=> >=)
            }
            Operator::LessThanOrEqual(source) => {
                let span = source.span();
                quote_spanned!(span=> <=)
            }

            Operator::Or(source) => {
                let span = source.span();
                quote_spanned!(span=> ||)
            }
            Operator::And(source) => {
                let span = source.span();
                quote_spanned!(span=> &&)
            }

            Operator::RangeInclusive(source) => {
                let span = source.span();
                quote_spanned!(span=> ..=)
            }
            Operator::RangeExclusive(source) => {
                let span = source.span();
                quote_spanned!(span=> ..)
            }
        });
    }
}

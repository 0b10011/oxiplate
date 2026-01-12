use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::char as nom_char;
use nom::combinator::{cut, into, not, opt, peek};
use nom::sequence::{pair, preceded, terminated};
use nom::{AsChar as _, Parser as _};
use quote::quote;

use crate::syntax::expression::{Expression, Res};
use crate::{Source, Tokens, internal_error};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Integer<'a>(Source<'a>);

impl<'a> Integer<'a> {
    /// Parse decimal literals (e.g., `19`).
    /// See: <https://doc.rust-lang.org/reference/tokens.html#integer-literals>
    fn parse_integer_literal(input: Source) -> Res<Source, Source> {
        let (input, number) = (
            take_while1(|char: char| char.is_ascii_digit()),
            take_while(|char: char| char.is_ascii_digit() || char == '_'),
        )
            .parse(input)?;

        Ok((
            input,
            number
                .0
                .merge(&number.1, "Digits/underscores should follow leading digits"),
        ))
    }

    /// Parse alternative base number literals with a `0b`, `0x`, and `0o` prefixes.
    /// Will fail if there's not at least one valid digit following the prefix.
    /// See: <https://doc.rust-lang.org/reference/tokens.html#integer-literals>
    fn parse_alternative_bases(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, prefix) = peek(preceded(
            nom_char('0'),
            alt((
                tag("b"),
                tag("x"),
                tag("o"),
                #[cfg(feature = "unreachable")]
                tag("q"),
            )),
        ))
        .parse(input)?;
        let (prefix, matcher) = match prefix.as_str() {
            "b" => ("0b", char::is_bin_digit as fn(char) -> bool),
            "x" => ("0x", char::is_hex_digit as fn(char) -> bool),
            "o" => ("0o", char::is_oct_digit as fn(char) -> bool),
            _ => internal_error!(prefix.span().unwrap(), "Unhandled alternative base prefix"),
        };

        let (input, number) = pair(
            tag(prefix),
            cut((
                take_while(|char: char| char == '_'),
                take_while1(|char: char| matcher(char)),
                take_while(|char: char| char == '_' || matcher(char)),
            )),
        )
        .parse(input)?;

        Ok((
            input,
            Self(
                number
                    .0
                    .merge(&number.1.0, "Leading underscores should follow prefix")
                    .merge(&number.1.1, "Digits should follow leading underscores")
                    .merge(&number.1.2, "Trailing underscores should follow digits"),
            ),
        ))
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        &self.0
    }

    pub(crate) fn to_tokens(&self) -> Tokens {
        let literal = ::syn::LitInt::new(self.0.as_str(), self.0.span());
        (quote! { #literal }, self.0.as_str().len())
    }
}

impl<'a> From<Integer<'a>> for Number<'a> {
    fn from(value: Integer<'a>) -> Self {
        Number::Integer(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Float<'a>(Source<'a>);

impl<'a> Float<'a> {
    pub(crate) fn source(&self) -> &Source<'a> {
        &self.0
    }

    pub(crate) fn to_tokens(&self) -> Tokens {
        let literal = ::syn::LitFloat::new(self.0.as_str(), self.0.span());
        (quote! { #literal }, self.0.as_str().len())
    }
}

impl<'a> From<Float<'a>> for Number<'a> {
    fn from(value: Float<'a>) -> Self {
        Number::Float(value)
    }
}

pub(crate) enum Number<'a> {
    Integer(Integer<'a>),
    Float(Float<'a>),
}

impl<'a> Number<'a> {
    /// Parse a number.
    /// See: <https://doc.rust-lang.org/reference/tokens.html#number-literals>
    pub(crate) fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        alt((into(Integer::parse_alternative_bases), Self::parse_decimal)).parse(input)
    }

    /// Parse decimal and floating-point literals.
    /// See: <https://doc.rust-lang.org/reference/tokens.html#integer-literals>
    /// See: <https://doc.rust-lang.org/reference/tokens.html#floating-point-literals>
    fn parse_decimal(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, integer) = Integer::parse_integer_literal.parse(input)?;

        // Decimal points won't exist for integers (with or without exponents).
        // E.g., `19` or `19e0`.
        let (input, point) = opt(terminated(tag("."), not(tag(".")))).parse(input)?;
        let Some(point) = point else {
            let (input, exponent) = opt(Self::parse_exponent).parse(input)?;

            return if let Some(exponent) = exponent {
                Ok((
                    input,
                    Float(integer.merge(&exponent, "Exponent should follow integer")).into(),
                ))
            } else {
                Ok((input, Integer(integer).into()))
            };
        };

        // Parse the fractional part of the decimal and the exponent, if any.
        let (input, end) = opt(pair(
            Integer::parse_integer_literal,
            opt(Self::parse_exponent),
        ))
        .parse(input)?;

        // If there's no fractional part or exponent,
        // the float (e.g., `19.`) can be returned early.
        let Some((fractional, exponent)) = end else {
            return Ok((
                input,
                Float(integer.merge(&point, "Decimal point should follow integer")).into(),
            ));
        };

        Ok((
            input,
            Float(
                integer
                    .merge(&point, "Decimal point should follow integer")
                    .merge(&fractional, "Fractional should follow decimal point")
                    .merge_some(exponent.as_ref(), "Exponent should follow fractional"),
            )
            .into(),
        ))
    }

    /// Parse float exponent (e.g., `e-1`, `E+2`, or `e3`).
    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-FLOAT_EXPONENT>
    fn parse_exponent(input: Source) -> Res<Source, Source> {
        let (input, (e, sign, separators, number)) = (
            alt((tag("e"), tag("E"))),
            opt(alt((tag("-"), tag("+")))),
            take_while(|char: char| char == '_'),
            cut(Integer::parse_integer_literal),
        )
            .parse(input)?;

        Ok((
            input,
            e.merge_some(sign.as_ref(), "Sign should follow 'e'")
                .merge(&separators, "Underscores should follow sign")
                .merge(&number, "Integer should follow underscores"),
        ))
    }
}

impl<'a> From<Number<'a>> for Expression<'a> {
    fn from(value: Number<'a>) -> Self {
        match value {
            Number::Integer(integer) => Expression::Integer(integer),
            Number::Float(float) => Expression::Float(float),
        }
    }
}

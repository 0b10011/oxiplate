use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{into, opt};
use proc_macro2::TokenStream;

use super::Pattern;
use crate::syntax::Res;
use crate::syntax::expression::{Char, Float, Integer, Number};
use crate::syntax::template::whitespace;
use crate::{Source, State};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Range<'a> {
    /// start..
    From { from: Value<'a>, source: Source<'a> },

    /// start..end
    Exclusive {
        from: Value<'a>,
        to: Value<'a>,
        source: Source<'a>,
    },

    /// start..=end
    Inclusive {
        from: Value<'a>,
        to: Value<'a>,
        source: Source<'a>,
    },

    /// ..end
    ExclusiveTo { to: Value<'a>, source: Source<'a> },

    /// ..=end
    InclusiveTo { to: Value<'a>, source: Source<'a> },
}

impl<'a> Range<'a> {
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        alt((
            Self::parse_from_or_exclusive,
            Self::parse_inclusive,
            Self::parse_exclusive_to,
            Self::parse_inclusive_to,
        ))
        .parse(input)
    }

    pub fn parse_from_or_exclusive(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, (from, leading_whitespace, operator, trailing_whitespace, to)) = (
            Value::parse,
            opt(whitespace),
            tag(".."),
            opt(whitespace),
            opt(Value::parse),
        )
            .parse(input)?;

        let source = from
            .source()
            .clone()
            .merge_some(
                leading_whitespace.as_ref(),
                "Whitespace expected after value",
            )
            .merge(&operator, "`..` operator expected after whitespace")
            .merge_some(
                trailing_whitespace.as_ref(),
                "Whitespace expected after `..` operator",
            )
            .merge_some(
                to.as_ref().map(Value::source),
                "Value expected after whitespace",
            );

        if let Some(to) = to {
            Ok((input, Self::Exclusive { from, to, source }))
        } else {
            Ok((input, Self::From { from, source }))
        }
    }

    pub fn parse_inclusive(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, (from, leading_whitespace, operator, trailing_whitespace, to)) = (
            Value::parse,
            opt(whitespace),
            tag("..="),
            opt(whitespace),
            Value::parse,
        )
            .parse(input)?;

        let source = from
            .source()
            .clone()
            .merge_some(
                leading_whitespace.as_ref(),
                "Whitespace expected after value",
            )
            .merge(&operator, "`..` operator expected after whitespace")
            .merge_some(
                trailing_whitespace.as_ref(),
                "Whitespace expected after `..` operator",
            )
            .merge(to.source(), "Value expected after whitespace");

        Ok((input, Self::Inclusive { from, to, source }))
    }

    pub fn parse_exclusive_to(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, (operator, trailing_whitespace, to)) =
            (tag(".."), opt(whitespace), Value::parse).parse(input)?;

        let source = operator
            .clone()
            .merge_some(
                trailing_whitespace.as_ref(),
                "Whitespace expected after `..` operator",
            )
            .merge(to.source(), "Value expected after whitespace");

        Ok((input, Self::ExclusiveTo { to, source }))
    }

    pub fn parse_inclusive_to(input: Source<'a>) -> Res<Source<'a>, Self> {
        let (input, (operator, trailing_whitespace, to)) =
            (tag("..="), opt(whitespace), Value::parse).parse(input)?;

        let source = operator
            .clone()
            .merge_some(
                trailing_whitespace.as_ref(),
                "Whitespace expected after `..` operator",
            )
            .merge(to.source(), "Value expected after whitespace");

        Ok((input, Self::InclusiveTo { to, source }))
    }

    pub fn source(&self) -> &Source<'a> {
        todo!("range source");
    }

    pub fn to_tokens(&self, _state: &State) -> TokenStream {
        todo!("range to_tokens");
    }
}

impl<'a> From<Range<'a>> for Pattern<'a> {
    fn from(value: Range<'a>) -> Self {
        Pattern::Range(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Value<'a> {
    Integer(Integer<'a>),
    Float(Float<'a>),
    Char(Char<'a>),
}

impl<'a> Value<'a> {
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        alt((into(Number::parse), into(Char::parse))).parse(input)
    }

    pub fn source(&self) -> &Source<'a> {
        match self {
            Self::Integer(integer) => integer.source(),
            Self::Float(float) => float.source(),
            Self::Char(char) => char.source(),
        }
    }
}

impl<'a> From<Number<'a>> for Value<'a> {
    fn from(value: Number<'a>) -> Self {
        match value {
            Number::Integer(integer) => Value::Integer(integer),
            Number::Float(float) => Value::Float(float),
        }
    }
}

impl<'a> From<Char<'a>> for Value<'a> {
    fn from(value: Char<'a>) -> Self {
        Value::Char(value)
    }
}

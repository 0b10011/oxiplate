use nom::Parser as _;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{into, opt};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use super::Pattern;
use crate::syntax::Res;
use crate::syntax::expression::{Char, Float, Integer, Number};
use crate::syntax::template::whitespace;
use crate::{Source, State};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Range<'a> {
    /// start..
    From {
        from: Value<'a>,
        operator: Source<'a>,
        source: Source<'a>,
    },

    /// start..end
    Exclusive {
        from: Value<'a>,
        operator: Source<'a>,
        to: Value<'a>,
        source: Source<'a>,
    },

    /// start..=end
    Inclusive {
        from: Value<'a>,
        operator: Source<'a>,
        to: Value<'a>,
        source: Source<'a>,
    },

    /// ..end
    ExclusiveTo {
        operator: Source<'a>,
        to: Value<'a>,
        source: Source<'a>,
    },

    /// ..=end
    InclusiveTo {
        operator: Source<'a>,
        to: Value<'a>,
        source: Source<'a>,
    },
}

impl<'a> Range<'a> {
    pub fn parse(input: Source<'a>) -> Res<Source<'a>, Self> {
        alt((
            Self::parse_inclusive,
            Self::parse_from_or_exclusive,
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
            Ok((
                input,
                Self::Exclusive {
                    from,
                    operator,
                    to,
                    source,
                },
            ))
        } else {
            Ok((
                input,
                Self::From {
                    from,
                    operator,
                    source,
                },
            ))
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

        Ok((
            input,
            Self::Inclusive {
                from,
                operator,
                to,
                source,
            },
        ))
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

        Ok((
            input,
            Self::ExclusiveTo {
                operator,
                to,
                source,
            },
        ))
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

        Ok((
            input,
            Self::InclusiveTo {
                operator,
                to,
                source,
            },
        ))
    }

    pub fn source(&self) -> &Source<'a> {
        match self {
            Self::From { source, .. }
            | Self::Exclusive { source, .. }
            | Self::Inclusive { source, .. }
            | Self::ExclusiveTo { source, .. }
            | Self::InclusiveTo { source, .. } => source,
        }
    }

    pub fn to_tokens(&self, _state: &State) -> TokenStream {
        let (from, operator, to) = match self {
            Self::From {
                from,
                operator,
                source: _,
            } => {
                let operator_span = operator.span();
                (Some(from), quote_spanned! {operator_span=> .. }, None)
            }
            Self::Exclusive {
                from,
                operator,
                to,
                source: _,
            } => {
                let operator_span = operator.span();
                (Some(from), quote_spanned! {operator_span=> .. }, Some(to))
            }
            Self::Inclusive {
                from,
                operator,
                to,
                source: _,
            } => {
                let operator_span = operator.span();
                (Some(from), quote_spanned! {operator_span=> ..= }, Some(to))
            }
            Self::ExclusiveTo {
                operator,
                to,
                source: _,
            } => {
                let operator_span = operator.span();
                (None, quote_spanned! {operator_span=> .. }, Some(to))
            }
            Self::InclusiveTo {
                operator,
                to,
                source: _,
            } => {
                let operator_span = operator.span();
                (None, quote_spanned! {operator_span=> ..= }, Some(to))
            }
        };

        let from = from.map_or_else(TokenStream::new, Value::to_tokens);
        let to = to.map_or_else(TokenStream::new, Value::to_tokens);

        quote! { #from #operator #to }
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

    pub fn to_tokens(&self) -> TokenStream {
        match self {
            Self::Integer(integer) => integer.to_tokens().0,
            Self::Float(float) => float.to_tokens().0,
            Self::Char(char) => char.to_tokens().0,
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

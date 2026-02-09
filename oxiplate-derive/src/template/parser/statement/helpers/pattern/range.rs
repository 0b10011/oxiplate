use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use super::Pattern;
use crate::parser::{alt, into, opt, take, Parser as _};
use crate::template::parser::expression::{Char, Float, Integer, Number};
use crate::template::parser::Res;
use crate::template::tokenizer::{TokenKind, TokenSlice};
use crate::{Source, State};

#[derive(Debug)]
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
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((
            Self::parse_inclusive,
            Self::parse_from_or_exclusive,
            Self::parse_exclusive_to,
            Self::parse_inclusive_to,
        ))
        .parse(tokens)
    }

    pub fn parse_from_or_exclusive(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (from, operator, to)) = (
            Value::parse,
            take(TokenKind::RangeExclusive),
            opt(Value::parse),
        )
            .parse(tokens)?;

        let source = from
            .source()
            .clone()
            .merge(operator.source(), "`..` operator expected after whitespace")
            .merge_some(
                to.as_ref().map(Value::source),
                "Value expected after whitespace",
            );

        if let Some(to) = to {
            Ok((
                tokens,
                Self::Exclusive {
                    from,
                    operator: operator.source().clone(),
                    to,
                    source,
                },
            ))
        } else {
            Ok((
                tokens,
                Self::From {
                    from,
                    operator: operator.source().clone(),
                    source,
                },
            ))
        }
    }

    pub fn parse_inclusive(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (from, operator, to)) =
            (Value::parse, take(TokenKind::RangeInclusive), Value::parse).parse(tokens)?;

        let source = from
            .source()
            .clone()
            .merge(operator.source(), "`..` operator expected after whitespace")
            .merge(to.source(), "Value expected after whitespace");

        Ok((
            tokens,
            Self::Inclusive {
                from,
                operator: operator.source().clone(),
                to,
                source,
            },
        ))
    }

    pub fn parse_exclusive_to(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (operator, to)) =
            (take(TokenKind::RangeExclusive), Value::parse).parse(tokens)?;

        let source = operator
            .source()
            .clone()
            .merge(to.source(), "Value expected after whitespace");

        Ok((
            tokens,
            Self::ExclusiveTo {
                operator: operator.source().clone(),
                to,
                source,
            },
        ))
    }

    pub fn parse_inclusive_to(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (operator, to)) =
            (take(TokenKind::RangeInclusive), Value::parse).parse(tokens)?;

        let source = operator
            .source()
            .clone()
            .merge(to.source(), "Value expected after whitespace");

        Ok((
            tokens,
            Self::InclusiveTo {
                operator: operator.source().clone(),
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
                let operator_span = operator.span_token();
                (Some(from), quote_spanned! {operator_span=> .. }, None)
            }
            Self::Exclusive {
                from,
                operator,
                to,
                source: _,
            } => {
                let operator_span = operator.span_token();
                (Some(from), quote_spanned! {operator_span=> .. }, Some(to))
            }
            Self::Inclusive {
                from,
                operator,
                to,
                source: _,
            } => {
                let operator_span = operator.span_token();
                (Some(from), quote_spanned! {operator_span=> ..= }, Some(to))
            }
            Self::ExclusiveTo {
                operator,
                to,
                source: _,
            } => {
                let operator_span = operator.span_token();
                (None, quote_spanned! {operator_span=> .. }, Some(to))
            }
            Self::InclusiveTo {
                operator,
                to,
                source: _,
            } => {
                let operator_span = operator.span_token();
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

#[derive(Debug)]
pub(crate) enum Value<'a> {
    Integer(Integer<'a>),
    Float(Float<'a>),
    Char(Char<'a>),
}

impl<'a> Value<'a> {
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((into(Number::parse), into(Char::parse))).parse(tokens)
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

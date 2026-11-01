use super::Pattern;
use crate::template::parser::expression::{Char, Float, Integer, Number};
use crate::template::parser::prelude::*;

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

    /// ..
    Full { operator: Source<'a> },
}

impl<'a> Range<'a> {
    pub(in crate::template::parser) fn parser() -> RangeParser<'a> {
        RangeParser {
            phantom_data: PhantomData,
        }
    }

    pub fn source(&self) -> &Source<'a> {
        match self {
            Self::From { source, .. }
            | Self::Exclusive { source, .. }
            | Self::Inclusive { source, .. }
            | Self::ExclusiveTo { source, .. }
            | Self::InclusiveTo { source, .. }
            | Self::Full { operator: source } => source,
        }
    }
}

impl<'a> ToTokensWithState<'a> for Range<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
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
            Self::Full { operator } => {
                let operator_span = operator.span_token();
                (None, quote_spanned! {operator_span=> .. }, None)
            }
        };

        let from = from.map_or_else(TokenStream::new, |value| {
            value.to_tokens_with_state(state).0
        });
        let to = to.map_or_else(TokenStream::new, |value| {
            value.to_tokens_with_state(state).0
        });

        (quote! { #from #operator #to }, EstimatedLength::new(0))
    }
}

impl<'a> From<Range<'a>> for Pattern<'a> {
    fn from(value: Range<'a>) -> Self {
        Pattern::Range(value)
    }
}

pub(in crate::template::parser) struct RangeParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> RangeParser<'a> {
    fn inclusive_parser() -> InclusiveParser<'a> {
        InclusiveParser {
            phantom_data: PhantomData,
        }
    }

    fn from_or_exclusive_parser() -> FromOrExclusiveParser<'a> {
        FromOrExclusiveParser {
            phantom_data: PhantomData,
        }
    }

    fn exclusive_to_parser() -> ExclusiveToParser<'a> {
        ExclusiveToParser {
            phantom_data: PhantomData,
        }
    }

    fn inclusive_to_parser() -> InclusiveToParser<'a> {
        InclusiveToParser {
            phantom_data: PhantomData,
        }
    }
}

impl<'a> Parser<'a> for RangeParser<'a> {
    type TokenKind = TokenKind;
    type Output = Range<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        alt((
            Self::inclusive_parser(),
            Self::from_or_exclusive_parser(),
            Self::exclusive_to_parser(),
            Self::inclusive_to_parser(),
        ))
        .parse(tokens)
    }
}

#[derive(Debug)]
pub(crate) enum Value<'a> {
    Integer(Integer<'a>),
    Float(Float<'a>),
    Char(Char<'a>),
}

impl<'a> Value<'a> {
    pub(in crate::template::parser) fn parser() -> ValueParser<'a> {
        ValueParser {
            phantom_data: PhantomData,
        }
    }

    pub fn source(&self) -> &Source<'a> {
        match self {
            Self::Integer(integer) => integer.source(),
            Self::Float(float) => float.source(),
            Self::Char(char) => char.source(),
        }
    }
}

impl<'a> ToTokensWithState<'a> for Value<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::Integer(integer) => integer.to_tokens_with_state(state),
            Self::Float(float) => float.to_tokens_with_state(state),
            Self::Char(char) => char.to_tokens_with_state(state),
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

pub(in crate::template::parser) struct ValueParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for ValueParser<'a> {
    type TokenKind = TokenKind;
    type Output = Value<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        alt((into(Number::parser()), into(Char::parser()))).parse(tokens)
    }
}

pub(in crate::template::parser) struct InclusiveParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for InclusiveParser<'a> {
    type TokenKind = TokenKind;
    type Output = Range<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (from, operator, to)) = (
            Value::parser(),
            take(TokenKind::RangeInclusive),
            Value::parser(),
        )
            .parse(tokens)?;

        let source = from
            .source()
            .clone()
            .merge(operator.source(), "`..` operator expected after whitespace")
            .merge(to.source(), "Value expected after whitespace");

        Ok((
            tokens,
            Range::Inclusive {
                from,
                operator: operator.source().clone(),
                to,
                source,
            },
        ))
    }
}

pub(in crate::template::parser) struct FromOrExclusiveParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for FromOrExclusiveParser<'a> {
    type TokenKind = TokenKind;
    type Output = Range<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (from, operator, to)) = (
            Value::parser(),
            take(TokenKind::RangeExclusive),
            ignore_all_errors(Value::parser()),
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
                Range::Exclusive {
                    from,
                    operator: operator.source().clone(),
                    to,
                    source,
                },
            ))
        } else {
            Ok((
                tokens,
                Range::From {
                    from,
                    operator: operator.source().clone(),
                    source,
                },
            ))
        }
    }
}

pub(in crate::template::parser) struct ExclusiveToParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for ExclusiveToParser<'a> {
    type TokenKind = TokenKind;
    type Output = Range<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (operator, to)) = (
            take(TokenKind::RangeExclusive),
            ignore_recoverable_errors(Value::parser()),
        )
            .parse(tokens)?;

        let Some(to) = to else {
            return Ok((
                tokens,
                Range::Full {
                    operator: operator.source().clone(),
                },
            ));
        };

        let source = operator
            .source()
            .clone()
            .merge(to.source(), "Value expected after whitespace");

        Ok((
            tokens,
            Range::ExclusiveTo {
                operator: operator.source().clone(),
                to,
                source,
            },
        ))
    }
}

pub(in crate::template::parser) struct InclusiveToParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for InclusiveToParser<'a> {
    type TokenKind = TokenKind;
    type Output = Range<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (operator, to)) =
            (take(TokenKind::RangeInclusive), Value::parser()).parse(tokens)?;

        let source = operator
            .source()
            .clone()
            .merge(to.source(), "Value expected after whitespace");

        Ok((
            tokens,
            Range::InclusiveTo {
                operator: operator.source().clone(),
                to,
                source,
            },
        ))
    }
}

use super::LiteralExpression;
use crate::template::parser::expression::Expression;
use crate::template::parser::prelude::*;

#[derive(Debug)]
pub(in crate::template::parser) struct Integer<'a> {
    source: &'a Source<'a>,
}

impl<'a> Integer<'a> {
    #[cfg(test)]
    pub(in crate::template::parser) fn new_for_test(source: &'a Source<'a>) -> Self {
        Self { source }
    }

    /// See: <https://doc.rust-lang.org/reference/tokens.html#number-literals>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, integer) = take(TokenKind::Integer).parse(tokens)?;

        Ok((
            tokens,
            Self {
                source: integer.source(),
            },
        ))
    }

    pub(in crate::template::parser) fn source(&self) -> &Source<'a> {
        self.source
    }
}

impl<'a> ToTokensWithState<'a> for Integer<'a> {
    fn to_tokens_with_state(&self, _state: &State<'a>) -> BuiltTokens {
        let literal = ::syn::LitInt::new(self.source.as_str(), self.source.span_token());
        (
            quote! { #literal },
            EstimatedLength::new(self.source.as_str().len()),
        )
    }
}

impl<'a> From<Integer<'a>> for Number<'a> {
    fn from(value: Integer<'a>) -> Self {
        Number::Integer(value)
    }
}

impl<'a> From<Integer<'a>> for LiteralExpression<'a> {
    fn from(value: Integer<'a>) -> Self {
        Self::Integer(value)
    }
}

#[derive(Debug)]
pub(in crate::template::parser) struct Float<'a> {
    source: Source<'a>,
}

impl<'a> Float<'a> {
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, float) = take(TokenKind::Float).parse(tokens)?;

        Ok((
            tokens,
            Self {
                source: float.source().clone(),
            },
        ))
    }

    pub(in crate::template::parser) fn source(&self) -> &Source<'a> {
        &self.source
    }
}

impl<'a> ToTokensWithState<'a> for Float<'a> {
    fn to_tokens_with_state(&self, _state: &State<'a>) -> BuiltTokens {
        let literal = ::syn::LitFloat::new(self.source.as_str(), self.source.span_token());
        (
            quote! { #literal },
            EstimatedLength::new(self.source.as_str().len()),
        )
    }
}

impl<'a> From<Float<'a>> for Number<'a> {
    fn from(value: Float<'a>) -> Self {
        Number::Float(value)
    }
}

impl<'a> From<Float<'a>> for LiteralExpression<'a> {
    fn from(value: Float<'a>) -> Self {
        Self::Float(value)
    }
}

pub(crate) enum Number<'a> {
    Integer(Integer<'a>),
    Float(Float<'a>),
}

impl<'a> Number<'a> {
    /// Parse a number.
    /// See: <https://doc.rust-lang.org/reference/tokens.html#number-literals>
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((into(Integer::parse), into(Float::parse))).parse(tokens)
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

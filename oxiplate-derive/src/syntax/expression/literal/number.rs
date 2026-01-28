use quote::quote;

use crate::parser::{Parser as _, alt, into, take};
use crate::syntax::expression::{Expression, Res};
use crate::tokenizer::parser::{TokenKind, TokenSlice};
use crate::{BuiltTokens, Source};

#[derive(Debug)]
pub(crate) struct Integer<'a> {
    source: &'a Source<'a>,
}

impl<'a> Integer<'a> {
    fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, integer) = take(TokenKind::Integer).parse(tokens)?;

        Ok((
            tokens,
            Self {
                source: integer.source(),
            },
        ))
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        self.source
    }

    pub(crate) fn to_tokens(&self) -> BuiltTokens {
        let literal = ::syn::LitInt::new(self.source.as_str(), self.source.span_token());
        (quote! { #literal }, self.source.as_str().len())
    }
}

impl<'a> From<Integer<'a>> for Number<'a> {
    fn from(value: Integer<'a>) -> Self {
        Number::Integer(value)
    }
}

#[derive(Debug)]
pub(crate) struct Float<'a> {
    source: Source<'a>,
}

impl<'a> Float<'a> {
    fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, float) = take(TokenKind::Float).parse(tokens)?;

        Ok((
            tokens,
            Self {
                source: float.source().clone(),
            },
        ))
    }

    pub(crate) fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub(crate) fn to_tokens(&self) -> BuiltTokens {
        let literal = ::syn::LitFloat::new(self.source.as_str(), self.source.span_token());
        (quote! { #literal }, self.source.as_str().len())
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

use super::Pattern;
use crate::template::parser::expression::{Bool, Char, Float, Integer, Number, String};
use crate::template::parser::prelude::*;

/// A literal value to match against.
/// See: <https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#matching-literals>
#[derive(Debug)]
pub(crate) enum Literal<'a> {
    Bool(Bool<'a>),
    Integer(Integer<'a>),
    Float(Float<'a>),
    String(String<'a>),
    Char(Char<'a>),
}

impl<'a> Literal<'a> {
    /// Parse a `Literal` from the input.
    pub fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        alt((
            into(Bool::parse),
            into(Number::parse),
            into(String::parse),
            into(Char::parse),
        ))
        .parse(tokens)
    }

    /// Get the underlying `Source` from the template for the literal and leading whitespace.
    pub fn source(&self) -> &Source<'a> {
        match self {
            Self::Bool(bool) => bool.source(),
            Self::Integer(integer) => integer.source(),
            Self::Float(float) => float.source(),
            Self::String(string) => string.source(),
            Self::Char(char) => char.source(),
        }
    }
}

impl<'a> ToTokensWithState<'a> for Literal<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::Bool(bool) => bool.to_tokens(),
            Self::Integer(integer) => integer.to_tokens(),
            Self::Float(float) => float.to_tokens(),
            Self::String(string) => string.to_tokens_with_state(state),
            Self::Char(char) => char.to_tokens_with_state(state),
        }
    }
}

impl<'a> From<Bool<'a>> for Literal<'a> {
    fn from(value: Bool<'a>) -> Self {
        Literal::Bool(value)
    }
}

impl<'a> From<Char<'a>> for Literal<'a> {
    fn from(value: Char<'a>) -> Self {
        Literal::Char(value)
    }
}

impl<'a> From<String<'a>> for Literal<'a> {
    fn from(value: String<'a>) -> Self {
        Literal::String(value)
    }
}

impl<'a> From<Number<'a>> for Literal<'a> {
    fn from(value: Number<'a>) -> Self {
        match value {
            Number::Integer(integer) => Literal::Integer(integer),
            Number::Float(float) => Literal::Float(float),
        }
    }
}

impl<'a> From<Literal<'a>> for Pattern<'a> {
    fn from(value: Literal<'a>) -> Self {
        Pattern::Literal(value)
    }
}

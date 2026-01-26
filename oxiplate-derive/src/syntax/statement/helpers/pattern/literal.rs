use proc_macro2::TokenStream;

use crate::Source;
use crate::syntax::Res;
use crate::syntax::expression::{Bool, Char, Float, Integer, Number, String};
use crate::syntax::parser::{Parser as _, alt, into};
use crate::syntax::statement::helpers::pattern::Pattern;
use crate::tokenizer::TokenSlice;

/// A literal value to match against.
/// See: <https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#matching-literals>
#[derive(Debug)]
pub enum Literal<'a> {
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

    /// Build the token stream for the literal pattern.
    pub fn to_tokens(&self) -> TokenStream {
        match self {
            Self::Bool(bool) => bool.to_tokens().0,
            Self::Integer(integer) => integer.to_tokens().0,
            Self::Float(float) => float.to_tokens().0,
            Self::String(string) => string.to_tokens().0,
            Self::Char(char) => char.to_tokens().0,
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

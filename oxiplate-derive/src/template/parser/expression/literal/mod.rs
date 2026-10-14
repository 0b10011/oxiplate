mod bool;
mod char;
mod number;
mod string;

pub(in crate::template::parser) use self::bool::Bool;
pub(in crate::template::parser) use self::char::Char;
pub(in crate::template::parser) use self::number::{Float, Integer, Number};
pub(in crate::template::parser) use self::string::String;
use crate::template::parser::prelude::*;

/// See: <https://doc.rust-lang.org/reference/expressions/literal-expr.html#railroad-LiteralExpression>
#[derive(Debug)]
pub(in crate::template::parser) enum LiteralExpression<'a> {
    Char(Char<'a>),
    String(String<'a>),
    #[expect(dead_code, reason = "Not yet implemented")]
    RawString,
    #[expect(dead_code, reason = "Not yet implemented")]
    Byte,
    #[expect(dead_code, reason = "Not yet implemented")]
    ByteString,
    #[expect(dead_code, reason = "Not yet implemented")]
    RawByteString,
    #[expect(dead_code, reason = "Not yet implemented")]
    CString,
    #[expect(dead_code, reason = "Not yet implemented")]
    RawCString,
    Integer(Integer<'a>),
    Float(Float<'a>),
    Bool(Bool<'a>),
}

impl<'a> LiteralExpression<'a> {
    /// See: <https://doc.rust-lang.org/reference/expressions/literal-expr.html#railroad-LiteralExpression>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, literal): (_, Self) = alt((
            into(Char::parse),
            into(String::parse),
            into(Integer::parse),
            into(Float::parse),
            into(Bool::parse),
        ))
        .parse(tokens)?;

        Ok((tokens, literal))
    }

    pub(in crate::template::parser) fn source(&self) -> &Source<'a> {
        match self {
            Self::Char(char) => char.source(),
            Self::String(string) => string.source(),
            Self::RawString
            | Self::Byte
            | Self::ByteString
            | Self::RawByteString
            | Self::CString
            | Self::RawCString => unreachable!("These literals are not yet implemented"),
            Self::Integer(integer) => integer.source(),
            Self::Float(float) => float.source(),
            Self::Bool(bool) => bool.source(),
        }
    }
}

impl<'a> ToTokensWithState<'a> for LiteralExpression<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::Char(char) => char.to_tokens_with_state(state),
            Self::String(string) => string.to_tokens_with_state(state),
            Self::RawString
            | Self::Byte
            | Self::ByteString
            | Self::RawByteString
            | Self::CString
            | Self::RawCString => unreachable!("These literals are not yet implemented"),
            Self::Integer(integer) => integer.to_tokens_with_state(state),
            Self::Float(float) => float.to_tokens_with_state(state),
            Self::Bool(bool) => bool.to_tokens_with_state(state),
        }
    }
}

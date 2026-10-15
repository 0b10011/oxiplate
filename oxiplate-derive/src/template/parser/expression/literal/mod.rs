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
    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-CHAR_LITERAL>
    Char(Char<'a>),

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-STRING_LITERAL>
    String(String<'a>),

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-RAW_STRING_LITERAL>
    #[expect(dead_code, reason = "Raw string literals aren't currently supported")]
    RawString,

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-BYTE_LITERAL>
    #[expect(dead_code, reason = "Byte literals aren't currently supported")]
    Byte,

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-BYTE_STRING_LITERAL>
    #[expect(dead_code, reason = "Byte string literals aren't currently supported")]
    ByteString,

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-RAW_BYTE_STRING_LITERAL>
    #[expect(
        dead_code,
        reason = "Raw byte string literals aren't currently supported"
    )]
    RawByteString,

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-C_STRING_LITERAL>
    #[expect(dead_code, reason = "C string literals aren't currently supported")]
    CString,

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-RAW_C_STRING_LITERAL>
    #[expect(dead_code, reason = "Raw C string literals aren't currently supported")]
    RawCString,

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-INTEGER_LITERAL>
    Integer(Integer<'a>),

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-FLOAT_LITERAL>
    Float(Float<'a>),

    /// `true` or `false`
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
            Self::RawString => unreachable!("Raw string literals aren't currently supported"),
            Self::Byte => unreachable!("Byte literals aren't currently supported"),
            Self::ByteString => unreachable!("Byte string literals aren't currently supported"),
            Self::RawByteString => {
                unreachable!("Raw byte string literals aren't currently supported")
            }
            Self::CString => unreachable!("C string literals aren't currently supported"),
            Self::RawCString => unreachable!("Raw C string literals aren't currently supported"),
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
            Self::RawString => unreachable!("Raw string literals aren't currently supported"),
            Self::Byte => unreachable!("Byte literals aren't currently supported"),
            Self::ByteString => unreachable!("Byte string literals aren't currently supported"),
            Self::RawByteString => {
                unreachable!("Raw byte string literals aren't currently supported")
            }
            Self::CString => unreachable!("C string literals aren't currently supported"),
            Self::RawCString => unreachable!("Raw C string literals aren't currently supported"),
            Self::Integer(integer) => integer.to_tokens_with_state(state),
            Self::Float(float) => float.to_tokens_with_state(state),
            Self::Bool(bool) => bool.to_tokens_with_state(state),
        }
    }
}

/// `LiteralExpression` with an optional leading `-`.
/// See: <https://doc.rust-lang.org/reference/expressions/literal-expr.html#railroad-LiteralExpression>
#[derive(Debug)]
pub(in crate::template::parser) struct PossiblyNegativeLiteralExpression<'a> {
    /// Optional `-`.
    sign: Option<Source<'a>>,
    literal: LiteralExpression<'a>,
}

impl<'a> PossiblyNegativeLiteralExpression<'a> {
    /// See: <https://doc.rust-lang.org/reference/expressions/literal-expr.html#railroad-LiteralExpression>
    pub(in crate::template::parser) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (sign, literal)) = (
            ignore_recoverable_errors(into(take(TokenKind::Minus))),
            LiteralExpression::parse,
        )
            .parse(tokens)?;

        Ok((tokens, Self { sign, literal }))
    }

    pub(in crate::template::parser) fn source(&self) -> Source<'a> {
        self.literal
            .source()
            .append_to_some(self.sign.clone(), "`-` expected before literal expression")
    }
}

impl<'a> ToTokensWithState<'a> for PossiblyNegativeLiteralExpression<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        if let Some(sign) = &self.sign {
            let span = sign.span_token();
            let (literal, estimated_length) = self.literal.to_tokens_with_state(state);
            (
                quote_spanned! {span=> - #literal },
                estimated_length + EstimatedLength::new(1),
            )
        } else {
            self.literal.to_tokens_with_state(state)
        }
    }
}

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
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "Raw string literals aren't currently supported")
    )]
    RawString,

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-BYTE_LITERAL>
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "Byte literals aren't currently supported")
    )]
    Byte,

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-BYTE_STRING_LITERAL>
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "Byte string literals aren't currently supported")
    )]
    ByteString,

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-RAW_BYTE_STRING_LITERAL>
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "Raw byte string literals aren't currently supported"
        )
    )]
    RawByteString,

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-C_STRING_LITERAL>
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "C string literals aren't currently supported")
    )]
    CString,

    /// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-RAW_C_STRING_LITERAL>
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "Raw C string literals aren't currently supported")
    )]
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
    pub(in crate::template::parser) fn parser() -> LiteralExpressionParser<'a> {
        LiteralExpressionParser {
            phantom_data: PhantomData,
        }
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

/// See: <https://doc.rust-lang.org/reference/expressions/literal-expr.html#railroad-LiteralExpression>
pub(in crate::template::parser) struct LiteralExpressionParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for LiteralExpressionParser<'a> {
    type TokenKind = TokenKind;
    type Output = LiteralExpression<'a>;

    /// See: <https://doc.rust-lang.org/reference/expressions/literal-expr.html#railroad-LiteralExpression>
    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, literal): (_, LiteralExpression) = alt((
            into(Char::parser()),
            into(String::parser()),
            into(Integer::parser()),
            into(Float::parser()),
            into(Bool::parser()),
        ))
        .parse(tokens)?;

        Ok((tokens, literal))
    }
}

/// `LiteralExpression` with an optional leading `-`.
/// Used in expressions and patterns.
/// See: <https://doc.rust-lang.org/reference/patterns.html#literal-patterns>
#[derive(Debug)]
pub(in crate::template::parser) struct PossiblyNegativeLiteralExpression<'a> {
    /// Optional `-`.
    sign: Option<Source<'a>>,
    literal: LiteralExpression<'a>,
}

impl<'a> PossiblyNegativeLiteralExpression<'a> {
    /// `LiteralExpression` with an optional leading `-`.
    /// Used in expressions and patterns.
    /// See: <https://doc.rust-lang.org/reference/patterns.html#literal-patterns>
    pub(in crate::template::parser) fn parser() -> PossiblyNegativeLiteralExpressionParser<'a> {
        PossiblyNegativeLiteralExpressionParser {
            phantom_data: PhantomData,
        }
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

/// `LiteralExpression` with an optional leading `-`.
/// Used in expressions and patterns.
/// See: <https://doc.rust-lang.org/reference/patterns.html#literal-patterns>
pub(in crate::template::parser) struct PossiblyNegativeLiteralExpressionParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for PossiblyNegativeLiteralExpressionParser<'a> {
    type TokenKind = TokenKind;
    type Output = PossiblyNegativeLiteralExpression<'a>;

    /// `LiteralExpression` with an optional leading `-`.
    /// Used in expressions and patterns.
    /// See: <https://doc.rust-lang.org/reference/patterns.html#literal-patterns>
    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        let (tokens, (sign, literal)) = (
            ignore_recoverable_errors(into(take(TokenKind::Minus))),
            LiteralExpression::parser(),
        )
            .parse(tokens)?;

        Ok((tokens, PossiblyNegativeLiteralExpression { sign, literal }))
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod test {
    use super::*;

    #[test]
    #[should_panic = "internal error: entered unreachable code: Byte literals aren't currently \
                      supported"]
    fn tokens_byte() {
        let _ = LiteralExpression::Byte.to_tokens_with_state(&State::new_for_test());
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Byte string literals aren't \
                      currently supported"]
    fn tokens_byte_string() {
        let _ = LiteralExpression::ByteString.to_tokens_with_state(&State::new_for_test());
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Raw byte string literals aren't \
                      currently supported"]
    fn tokens_raw_byte_string() {
        let _ = LiteralExpression::RawByteString.to_tokens_with_state(&State::new_for_test());
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: C string literals aren't currently \
                      supported"]
    fn tokens_c_string() {
        let _ = LiteralExpression::CString.to_tokens_with_state(&State::new_for_test());
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Raw C string literals aren't \
                      currently supported"]
    fn tokens_raw_c_string() {
        let _ = LiteralExpression::RawCString.to_tokens_with_state(&State::new_for_test());
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Raw string literals aren't \
                      currently supported"]
    fn tokens_raw_string() {
        let _ = LiteralExpression::RawString.to_tokens_with_state(&State::new_for_test());
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Byte literals aren't currently \
                      supported"]
    fn source_byte() {
        let _ = LiteralExpression::Byte.source();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Byte string literals aren't \
                      currently supported"]
    fn source_byte_string() {
        let _ = LiteralExpression::ByteString.source();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Raw byte string literals aren't \
                      currently supported"]
    fn source_raw_byte_string() {
        let _ = LiteralExpression::RawByteString.source();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: C string literals aren't currently \
                      supported"]
    fn source_c_string() {
        let _ = LiteralExpression::CString.source();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Raw C string literals aren't \
                      currently supported"]
    fn source_raw_c_string() {
        let _ = LiteralExpression::RawCString.source();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: Raw string literals aren't \
                      currently supported"]
    fn source_raw_string() {
        let _ = LiteralExpression::RawString.source();
    }
}

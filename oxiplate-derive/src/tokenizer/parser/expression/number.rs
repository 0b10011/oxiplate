use super::Token;
use crate::Source;
use crate::syntax::UnexpectedTokenError;
use crate::tokenizer::ParseError;
use crate::tokenizer::buffered_source::BufferedSource;
use crate::tokenizer::parser::{Res, TokenKind};

/// Parse decimal and floating-point literals.
/// See: <https://doc.rust-lang.org/reference/tokens.html#integer-literals>
/// See: <https://doc.rust-lang.org/reference/tokens.html#floating-point-literals>
pub fn consume_decimal<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> Res<'a> {
    // Match the rest of the integer
    source.next_while(|char| matches!(char, '_' | '0'..='9'));

    // Decimal points won't exist for integers (with or without exponents).
    // E.g., `19` or `19e0`.
    let kind = if source.peek() == Some('.') && source.peek_2() != Some(['.', '.']) {
        // Match the `.`
        let _ = source.next();

        // Parse the fractional part of the decimal and the exponent, if any.
        #[allow(clippy::manual_is_ascii_check)]
        if source.expect(|char| matches!(char, '0'..='9')).is_err() {
            let source = source
                .consume()
                .expect("At least one digit and `.` already consumed");
            return (
                None,
                Ok(Token::new(TokenKind::Float, &source, leading_whitespace)),
            );
        }
        source.next_while(|char| matches!(char, '_' | '0'..='9'));

        match parse_exponent(source) {
            Ok(_) => (),
            Err(parse_error) => {
                return (
                    None,
                    Err(UnexpectedTokenError::new(
                        parse_error.message(),
                        source
                            .consume()
                            .expect("At least one digit was parsed")
                            .append_to_leading_whitespace(
                                leading_whitespace,
                                "Number expected after whitespace",
                            ),
                    )),
                );
            }
        }

        TokenKind::Float
    } else {
        match parse_exponent(source) {
            Ok(true) => TokenKind::Float,
            Ok(false) => TokenKind::Integer,
            Err(parse_error) => {
                return (
                    None,
                    Err(UnexpectedTokenError::new(
                        parse_error.message(),
                        source
                            .consume()
                            .expect("At least one digit was parsed")
                            .append_to_leading_whitespace(
                                leading_whitespace,
                                "Number expected after leading whitespace",
                            ),
                    )),
                );
            }
        }
    };

    (
        None,
        Ok(Token::new(
            kind,
            &source.consume().expect("At least one digit was parsed"),
            leading_whitespace,
        )),
    )
}

/// Parse float exponent (e.g., `e-1`, `E+2`, or `e3`).
/// See: <https://doc.rust-lang.org/reference/tokens.html#railroad-FLOAT_EXPONENT>
fn parse_exponent(source: &mut BufferedSource) -> Result<bool, ParseError> {
    if !source.next_if(|char| matches!(char, 'e' | 'E')) {
        return Ok(false);
    }

    source.next_if(|char| matches!(char, '-' | '+'));
    source.next_while(is_underscore);
    #[allow(clippy::manual_is_ascii_check)]
    let Ok(()) = source.expect(|char| matches!(char, '0'..='9')) else {
        return Err(ParseError::new("Expected at least one digit in exponent"));
    };
    source.next_while(|char| matches!(char, '_' | '0'..='9'));

    Ok(true)
}

/// Parse alternative base number literals with a `0b`, `0x`, and `0o` prefixes.
/// Will fail if there's not at least one valid digit following the prefix.
/// See: <https://doc.rust-lang.org/reference/tokens.html#integer-literals>
pub fn consume_alternative_base<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> Res<'a> {
    let Some(char) = source.peek() else {
        let source = source.consume().expect("Buffer should contain `0`");
        return (
            None,
            Ok(Token::new(TokenKind::Integer, &source, leading_whitespace)),
        );
    };

    let digit_callback = match char {
        // Decimal numbers can have any number of leading zeroes,
        // but alternative bases can't.
        '0'..='9' => return consume_decimal(source, leading_whitespace),
        'b' => is_bin_digit,
        'x' => is_hex_digit,
        'o' => is_oct_digit,
        'a'..='z' | 'A'..='Z' | '_' => {
            // Match other suffix characters
            source.next_while(|char| matches!(char, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_'));

            let source = source.consume().expect("Buffer should contain `0`");

            return (
                None,
                Err(UnexpectedTokenError::new(
                    "Unexpected suffix",
                    source.append_to_leading_whitespace(
                        leading_whitespace,
                        "Number expected after leading whitespace",
                    ),
                )),
            );
        }
        _ => {
            let source = source.consume().expect("Buffer should contain `0`");
            return (
                None,
                Ok(Token::new(TokenKind::Integer, &source, leading_whitespace)),
            );
        }
    };

    // Claim the char checked with `peek()`
    let _ = source.next();

    // Any number of underscores is allowed
    // between the prefix and value.
    source.next_while(is_underscore);

    // Ensure at least one digit exists
    if source.next_while(digit_callback) == 0 {
        let number = source
            .consume()
            .expect("Buffer should already contain prefix and maybe underscores");

        return (
            None,
            Err(UnexpectedTokenError::new(
                "At least one digit expected after alternative base prefix",
                number.append_to_leading_whitespace(
                    leading_whitespace,
                    "Number expected after leading whitespace",
                ),
            )),
        );
    }

    // Match the rest of the underscores and valid digits
    while source.next_while(is_underscore) > 0 && source.next_while(digit_callback) > 0 {}

    let number = source
        .consume()
        .expect("Buffer should already contain prefix, underscores, and at least one digit");

    (
        None,
        Ok(Token::new(TokenKind::Integer, &number, leading_whitespace)),
    )
}

#[inline]
fn is_underscore(char: char) -> bool {
    char == '_'
}

#[inline]
fn is_bin_digit(char: char) -> bool {
    matches!(char, '0'..='1')
}

#[allow(clippy::manual_is_ascii_check)]
#[inline]
fn is_hex_digit(char: char) -> bool {
    matches!(char, '0'..='9' | 'a'..='f' | 'A'..='F')
}

#[inline]
fn is_oct_digit(char: char) -> bool {
    matches!(char, '0'..='7')
}

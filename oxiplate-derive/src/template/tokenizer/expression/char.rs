use super::Token;
use crate::Source;
use crate::template::tokenizer::Res;
use crate::template::tokenizer::kind::TokenKind;
use crate::tokenizer::{BufferedSource, ParseError, UnexpectedTokenError};

/// Chars and lifetimes both start with `'`.
/// This helps differentiate them
/// since they're parsed at the same time.
enum CharOrLifetime<'a> {
    Char(char),
    Lifetime(Source<'a>),
}

/// Parse char literal (e.g., `'a'`) or lifetime (e.g., `'a`).
/// See: <https://doc.rust-lang.org/reference/tokens.html#character-literals>
/// See: <https://doc.rust-lang.org/reference/tokens.html#lifetimes-and-loop-labels>
fn parse_char_or_lifetime<'a>(
    source: &mut BufferedSource<'a>,
) -> Result<CharOrLifetime<'a>, ParseError> {
    macro_rules! error {
        ($source:ident, $error:literal) => {{
            parse_char_end($source)?;
            return Err(ParseError::new($error));
        }};
    }

    let char = match source.next() {
        Some('\\') => match source.next() {
            Some(char @ ('\'' | '"' | '\\')) => char,
            Some('n') => '\n',
            Some('r') => '\r',
            Some('t') => '\t',
            Some('0') => '\0',
            Some(_char) => error!(
                source,
                r#"Unknown character escape. Expected `\\`, `\"`, `\'`, `\n`, `\r`, `\t`, or `\0`"#
            ),
            None => error!(
                source,
                "End of file encountered while parsing a character literal"
            ),
        },

        // Allow raw newlines, carriage returns, and tabs
        // to avoid having to double escape them in inline templates.
        // they'll be output as escapes in the final template
        // to prevent Rust from complaining.
        Some('\n') => '\n',
        Some('\r') => '\r',
        Some('\t') => '\t',

        Some('\'') => {
            return Err(ParseError::new(
                "No character specified in the character literal",
            ));
        }

        Some(char) if source.peek() == Some('\'') => char,

        Some('a'..='z' | 'A'..='Z' | '_') if source.peek().is_some() => {
            source.next_while(|char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));

            match source.peek() {
                Some('\'') => {
                    let _ = source.next();
                    return Err(ParseError::new(
                        r#"More than one character present in character literal. Consider using `"` instead of `'` to use a string literal instead."#,
                    ));
                }
                None => {
                    return Err(ParseError::new(
                        "End of file encountered while parsing a lifetime.",
                    ));
                }
                _ => (),
            }

            return Ok(CharOrLifetime::Lifetime(
                source
                    .consume()
                    .expect("Buffer should be at least one character"),
            ));
        }

        // Error will get caught by `parse_char_end()`
        Some(char) => char,

        None => error!(
            source,
            "End of file encountered while parsing a character literal"
        ),
    };

    parse_char_end(source).map(|()| CharOrLifetime::Char(char))
}

fn parse_char_end(source: &mut BufferedSource) -> Result<(), ParseError> {
    match source.next() {
        Some('\'') => Ok(()),
        Some(_) => {
            // Match any extra chars
            source.next_until(|char| char == '\'');

            // Match `'` if not EOF
            if source.next_if(|char| char == '\'') {
                Err(ParseError::new(
                    r#"More than one character present in character literal. Consider using `"` instead of `'` to use a string literal instead."#,
                ))
            } else {
                Err(ParseError::new(
                    "Unclosed character literal. Expected `'` after first character",
                ))
            }
        }
        None => Err(ParseError::new(
            "End of file encountered while parsing a character literal",
        )),
    }
}

/// Parse and consume a char literal (e.g., `'a'`) or lifetime (e.g., `'a`).
/// See: <https://doc.rust-lang.org/reference/tokens.html#character-literals>
/// See: <https://doc.rust-lang.org/reference/tokens.html#lifetimes-and-loop-labels>
pub fn consume_char_or_lifetime<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> Res<'a> {
    match parse_char_or_lifetime(source) {
        Ok(CharOrLifetime::Char(char)) => {
            let source = source
                .consume()
                .expect("Buffer should contain `'` at least");

            (
                None,
                Ok(Token::new(
                    TokenKind::Char(char),
                    &source,
                    leading_whitespace,
                )),
            )
        }
        Ok(CharOrLifetime::Lifetime(source)) => (
            None,
            Ok(Token::new(TokenKind::Lifetime, &source, leading_whitespace)),
        ),
        Err(parse_error) => {
            let source = source
                .consume()
                .expect("Buffer should contain `'` at least");

            (
                None,
                Err(UnexpectedTokenError::new(parse_error.message(), source)),
            )
        }
    }
}

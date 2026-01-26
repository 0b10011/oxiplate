use crate::Source;
use crate::tokenizer::buffered_source::BufferedSource;
use crate::tokenizer::{Context, ParseError, Token, TokenKind};

/// Parse char literal (e.g., `'a'`).
/// See: <https://doc.rust-lang.org/reference/tokens.html#character-literals>
fn parse_char(source: &mut BufferedSource) -> Result<char, ParseError> {
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

        Some(char) => char,
        None => error!(
            source,
            "End of file encountered while parsing a character literal"
        ),
    };

    parse_char_end(source).map(|()| char)
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

pub fn consume_char<'a>(
    source: &mut BufferedSource<'a>,
    leading_whitespace: Option<Source<'a>>,
) -> (Option<Context>, Token<'a>) {
    match parse_char(source) {
        Ok(char) => {
            let source = source
                .consume()
                .expect("Buffer should contain `\"` at least");

            (
                None,
                Token::new(TokenKind::Char(char), &source, leading_whitespace),
            )
        }
        Err(parse_error) => {
            let source = source
                .consume()
                .expect("Buffer should contain `'` at least");

            (
                None,
                Token::new(
                    TokenKind::Unexpected(Box::new(parse_error)),
                    &source,
                    leading_whitespace,
                ),
            )
        }
    }
}
